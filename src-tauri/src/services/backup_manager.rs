use chrono::{Duration, NaiveDateTime, Utc};
use keyring::Entry;
use sqlx::SqlitePool;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::{
    dtos::{
        backup_dto::{BackupResultDto, BackupSettingsDto, BackupSettingsRequestDto},
        backup_settings_file_dto::BackupSettingsFileDto,
    },
    models::backup::BackupSettings,
    services::backup_service,
    utils::backup_lock,
};

const MAX_BACKUPS: usize = 5;
const PASSWORD_SERVICE: &str = "personal_trainer_project.backup";

fn profile() -> &'static str {
    if cfg!(debug_assertions) {
        "dev"
    } else {
        "release"
    }
}

fn settings_path(data_dir: &Path) -> PathBuf {
    data_dir.join(format!("backup-settings-{}.json", profile()))
}

fn read_settings(data_dir: &Path) -> Result<Option<BackupSettings>, String> {
    let bytes = match fs::read(settings_path(data_dir)) {
        Ok(bytes) => bytes,

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        }

        Err(_) => {
            return Err("Não foi possível ler a configuração do backup.".to_string());
        }
    };

    let dto: BackupSettingsFileDto = serde_json::from_slice(&bytes)
        .map_err(|_| "A configuração do backup está inválida.".to_string())?;

    Ok(Some(BackupSettings::from(dto)))
}

fn write_settings(data_dir: &Path, settings: &BackupSettings) -> Result<(), String> {
    fs::create_dir_all(data_dir)
        .map_err(|_| "Não foi possível acessar a pasta do aplicativo.".to_string())?;

    let dto = BackupSettingsFileDto::from(settings);

    let bytes = serde_json::to_vec_pretty(&dto)
        .map_err(|_| "Não foi possível preparar a configuração do backup.".to_string())?;

    let mut temporary = NamedTempFile::new_in(data_dir)
        .map_err(|_| "Não foi possível preparar o arquivo de configuração.".to_string())?;

    temporary
        .write_all(&bytes)
        .map_err(|_| "Não foi possível gravar a configuração do backup.".to_string())?;

    temporary
        .as_file()
        .sync_all()
        .map_err(|_| "Não foi possível confirmar a configuração do backup.".to_string())?;

    let saved = temporary
        .persist(settings_path(data_dir))
        .map_err(|_| "Não foi possível salvar a configuração do backup.".to_string())?;

    drop(saved);

    Ok(())
}

pub fn get_settings(data_dir: &Path) -> Result<Option<BackupSettingsDto>, String> {
    Ok(read_settings(data_dir)?.map(BackupSettingsDto::from))
}

pub fn configure(
    data_dir: &Path,
    dto: BackupSettingsRequestDto,
) -> Result<BackupSettingsDto, String> {
    let _guard = backup_lock::acquire()?;

    let folder = PathBuf::from(&dto.folder);

    if !folder.is_absolute() || !folder.is_dir() {
        return Err("Selecione uma pasta existente.".to_string());
    }

    let probe = NamedTempFile::new_in(&folder)
        .map_err(|_| "Não foi possível escrever na pasta escolhida.".to_string())?;

    drop(probe);

    let folder = folder.to_string_lossy().into_owned();
    let previous = read_settings(data_dir)?;
    let password = dto.password.filter(|value| !value.is_empty());

    let mut new_account = None;

    let password_account = match password {
        Some(password) => {
            if password.chars().count() < 12 {
                return Err(
                    "Use uma senha de recuperação com pelo menos 12 caracteres.".to_string()
                );
            }

            let account = format!("{}-{}", profile(), Uuid::new_v4(),);

            let entry = Entry::new(PASSWORD_SERVICE, &account)
                .map_err(|_| "Não foi possível acessar o cofre do sistema.".to_string())?;

            entry
                .set_password(&password)
                .map_err(|_| "Não foi possível guardar a senha de recuperação.".to_string())?;

            new_account = Some(account.clone());

            account
        }

        None => previous
            .as_ref()
            .map(|settings| settings.password_account.clone())
            .ok_or_else(|| "Informe a senha de recuperação do backup.".to_string())?,
    };

    let last_automatic_backup = previous
        .as_ref()
        .filter(|settings| settings.folder == folder)
        .and_then(|settings| settings.last_automatic_backup);

    let settings = BackupSettings {
        folder,
        password_account,
        last_automatic_backup,
    };

    if let Err(error) = write_settings(data_dir, &settings) {
        if let Some(account) = new_account {
            if let Ok(entry) = Entry::new(PASSWORD_SERVICE, &account) {
                let _ = entry.delete_credential();
            }
        }

        return Err(error);
    }

    if let Some(previous) = previous {
        if previous.password_account != settings.password_account {
            if let Ok(entry) = Entry::new(PASSWORD_SERVICE, &previous.password_account) {
                let _ = entry.delete_credential();
            }
        }
    }

    Ok(BackupSettingsDto::from(settings))
}

pub async fn create_manual(pool: &SqlitePool, data_dir: &Path) -> Result<BackupResultDto, String> {
    let _guard = backup_lock::acquire()?;

    let settings = read_settings(data_dir)?
        .ok_or_else(|| "Configure a pasta e a senha do backup primeiro.".to_string())?;

    create(pool, data_dir, settings, false).await
}

pub async fn create_automatic_if_due(
    pool: &SqlitePool,
    data_dir: &Path,
) -> Result<Option<BackupResultDto>, String> {
    let _guard = backup_lock::acquire()?;

    let Some(settings) = read_settings(data_dir)? else {
        return Ok(None);
    };

    if let Some(last_backup) = settings.last_automatic_backup {
        let elapsed = Utc::now().signed_duration_since(last_backup);

        if elapsed < Duration::days(7) {
            return Ok(None);
        }
    }

    create(pool, data_dir, settings, true).await.map(Some)
}

async fn create(
    pool: &SqlitePool,
    data_dir: &Path,
    mut settings: BackupSettings,
    automatic: bool,
) -> Result<BackupResultDto, String> {
    let folder = PathBuf::from(&settings.folder);

    if !folder.is_dir() {
        return Err("A pasta configurada para os backups não está disponível.".to_string());
    }

    let entry = Entry::new(PASSWORD_SERVICE, &settings.password_account)
        .map_err(|_| "Não foi possível acessar o cofre do sistema.".to_string())?;

    let password = entry.get_password().map_err(|_| {
        "Não foi possível recuperar a senha do backup. Configure-a novamente.".to_string()
    })?;

    let filename = format!(
        "pt-backup-{}-{}_{}.ptbackup",
        profile(),
        Utc::now().format("%Y%m%dT%H%M%S%3fZ"),
        Uuid::new_v4(),
    );

    let destination = folder.join(filename);

    backup_service::create_backup(pool, data_dir, &destination, password).await?;

    let mut warnings = Vec::new();

    if automatic {
        settings.last_automatic_backup = Some(Utc::now());

        if let Err(error) = write_settings(data_dir, &settings) {
            warnings.push(format!(
                "O backup foi criado, mas a data do automático não foi salva: {error}"
            ));
        }
    }

    if let Err(error) = remove_old_backups(&folder, &destination) {
        warnings.push(format!(
            "O backup foi criado, mas a limpeza não foi concluída: {error}"
        ));
    }

    Ok(BackupResultDto {
        path: destination.to_string_lossy().into_owned(),
        warning: if warnings.is_empty() {
            None
        } else {
            Some(warnings.join(" "))
        },
    })
}

pub(crate) fn remove_old_backups(folder: &Path, newest: &Path) -> Result<(), String> {
    let prefix = format!("pt-backup-{}-", profile());

    let entries = fs::read_dir(folder)
        .map_err(|_| "Não foi possível listar os backups da pasta.".to_string())?;

    let mut backups = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|_| "Não foi possível ler um arquivo da pasta.".to_string())?;

        let file_type = entry
            .file_type()
            .map_err(|_| "Não foi possível identificar um arquivo da pasta.".to_string())?;

        if !file_type.is_file() {
            continue;
        }

        let name = entry.file_name();

        let Some(name) = name.to_str() else {
            continue;
        };

        let Some(body) = name
            .strip_prefix(&prefix)
            .and_then(|value| value.strip_suffix(".ptbackup"))
        else {
            continue;
        };

        let Some((timestamp, identifier)) = body.split_once('_') else {
            continue;
        };

        if Uuid::parse_str(identifier).is_err() {
            continue;
        }

        let Ok(created_at) = NaiveDateTime::parse_from_str(timestamp, "%Y%m%dT%H%M%S%3fZ") else {
            continue;
        };

        backups.push((created_at, entry.path()));
    }

    backups.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));

    let excess = backups.len().saturating_sub(MAX_BACKUPS);

    for (_, path) in backups
        .into_iter()
        .filter(|(_, path)| path.as_path() != newest)
        .take(excess)
    {
        fs::remove_file(&path).map_err(|_| {
            format!(
                "Não foi possível remover o backup antigo: {}",
                path.display(),
            )
        })?;
    }

    Ok(())
}
