use age::{secrecy::SecretString, Decryptor};
use keyring::Entry;
use sqlx::SqlitePool;
use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

use crate::{
    dtos::backup_package_dto::BackupPackageDto,
    models::{backup::BackupPackage, backup_photo::BackupPhoto},
    repositories::backup_repository::BackupRepository,
    utils::{
        assessment_photo_files::{self, run_file_task},
        assessment_photo_key,
    },
};

const MAX_PACKAGE_SIZE: u64 = 256 * 1024 * 1024;

pub async fn restore_backup(
    pool: &SqlitePool,
    data_dir: &Path,
    backup_path: &Path,
    password: String,
) -> Result<String, String> {
    if pool.is_closed() {
        return Err("Reinicie o aplicativo antes de restaurar.".to_string());
    }

    let filename = if cfg!(debug_assertions) {
        "personal-dev.db"
    } else {
        "personal.db"
    };

    let database_path = data_dir.join(filename);

    let entry = Entry::new("personal_trainer_project.database", filename)
        .map_err(|_| "Não foi possível acessar o cofre do sistema.".to_string())?;

    let local_key = entry
        .get_password()
        .map_err(|_| "Não foi possível recuperar a chave local do banco.".to_string())?;

    if !valid_key(&local_key) {
        return Err("A chave local possui formato inválido.".to_string());
    }

    let (temporary, package) = decrypt_backup(data_dir, backup_path, password).await?;

    prepare_restored_database(pool, temporary.path(), &package.database_key, &local_key).await?;

    restore_photos(pool, data_dir, temporary.path(), &local_key, package.photos).await?;

    pool.validate_database(temporary.path(), &local_key).await?;

    temporary
        .as_file()
        .sync_all()
        .map_err(|_| "Não foi possível confirmar o banco restaurado.".to_string())?;

    // Após fechar o pool, o aplicativo precisará reiniciar.
    pool.close_for_restore(&database_path, &local_key).await?;

    replace_database(temporary, database_path).await
}

pub async fn prepare_restored_database<R: BackupRepository>(
    repository: &R,
    path: &Path,
    imported_key: &str,
    local_key: &str,
) -> Result<(), String> {
    if !valid_key(imported_key) || !valid_key(local_key) {
        return Err("A chave do banco possui formato inválido.".to_string());
    }

    repository.validate_database(path, imported_key).await?;

    let tables = repository.count_required_tables(path, imported_key).await?;

    if tables != 8 {
        return Err("O backup não contém todas as tabelas esperadas.".to_string());
    }

    repository.run_migrations(path, imported_key).await?;

    repository.validate_database(path, imported_key).await?;

    repository
        .change_database_key(path, imported_key, local_key)
        .await?;

    repository.validate_database(path, local_key).await?;

    Ok(())
}

pub async fn restore_photos<R: BackupRepository>(
    repository: &R,
    data_dir: &Path,
    database_path: &Path,
    database_key: &str,
    photos: Vec<BackupPhoto>,
) -> Result<(), String> {
    let expected = repository
        .find_photo_paths(database_path, database_key)
        .await?
        .into_iter()
        .collect::<HashSet<_>>();

    let mut received = HashSet::new();

    for photo in &photos {
        if !received.insert(photo.file_path.clone()) {
            return Err("O backup contém fotos duplicadas.".to_string());
        }
    }

    if received != expected {
        return Err(
            "As fotos do backup não correspondem às fotos referenciadas pelo banco. A restauração foi interrompida."
                .to_string(),
        );
    }

    if photos.is_empty() {
        return Ok(());
    }

    let directory = data_dir.to_path_buf();

    let key = run_file_task(move || assessment_photo_key::load_or_create_key(&directory)).await?;

    let mut created_paths = Vec::new();

    let result: Result<(), String> = async {
        for photo in photos {
            let directory = data_dir.to_path_buf();
            let previous_path = photo.file_path;
            let image_hex = photo.image_hex;

            let new_path = run_file_task(move || {
                let bytes = hex::decode(image_hex)
                    .map_err(|_| "Uma foto do backup possui conteúdo inválido.".to_string())?;

                assessment_photo_files::encrypt_photo_bytes(&directory, &bytes, &key)
            })
            .await?;

            created_paths.push(new_path.clone());

            repository
                .update_photo_path(database_path, database_key, previous_path, new_path)
                .await?;
        }

        Ok(())
    }
    .await;

    if let Err(error) = result {
        let directory = data_dir.to_path_buf();

        let cleanup = run_file_task(move || {
            let mut failures = Vec::new();

            for path in created_paths {
                if let Err(error) = assessment_photo_files::remove_photo(&directory, &path) {
                    failures.push(error);
                }
            }

            if failures.is_empty() {
                Ok(())
            } else {
                Err(failures.join("; "))
            }
        })
        .await;

        return Err(match cleanup {
            Ok(()) => error,

            Err(cleanup_error) => {
                format!("{error} A limpeza das fotos também falhou: {cleanup_error}")
            }
        });
    }

    Ok(())
}

fn valid_key(key: &str) -> bool {
    key.len() == 64 && key.bytes().all(|byte| byte.is_ascii_hexdigit())
}

async fn decrypt_backup(
    data_dir: &Path,
    backup_path: &Path,
    password: String,
) -> Result<(NamedTempFile, BackupPackage), String> {
    let data_dir = data_dir.to_path_buf();
    let backup_path = backup_path.to_path_buf();
    let password = SecretString::from(password);

    tauri::async_runtime::spawn_blocking(move || {
        let file = File::open(&backup_path)
            .map_err(|_| "Não foi possível abrir o arquivo de backup.".to_string())?;

        let size = file
            .metadata()
            .map_err(|_| "Não foi possível verificar o tamanho do backup.".to_string())?
            .len();

        if size > MAX_PACKAGE_SIZE + 1024 * 1024 {
            return Err("O arquivo de backup ultrapassa o limite permitido.".to_string());
        }

        let decryptor = Decryptor::new(file)
            .map_err(|_| "O arquivo selecionado não é um backup válido.".to_string())?;

        if !decryptor.is_scrypt() {
            return Err("Este backup não utiliza senha de recuperação.".to_string());
        }

        let mut identity = age::scrypt::Identity::new(password);
        identity.set_max_work_factor(20);

        let reader = decryptor
            .decrypt(std::iter::once(&identity as &dyn age::Identity))
            .map_err(|_| "Senha incorreta ou backup incompatível.".to_string())?;

        let mut plaintext = Vec::new();

        reader
            .take(MAX_PACKAGE_SIZE + 1)
            .read_to_end(&mut plaintext)
            .map_err(|_| "O backup está incompleto ou foi alterado.".to_string())?;

        if plaintext.len() as u64 > MAX_PACKAGE_SIZE {
            return Err("O conteúdo do backup excede o limite permitido.".to_string());
        }

        let dto: BackupPackageDto = serde_json::from_slice(&plaintext)
            .map_err(|_| "O conteúdo do backup é inválido.".to_string())?;

        drop(plaintext);

        let mut package = BackupPackage::from(dto);

        if !matches!(package.format_version, 1 | 2)
            || package.application != "personal_trainer_project"
        {
            return Err("Formato de backup incompatível com este aplicativo.".to_string());
        }

        if package.format_version == 1 && !package.photos.is_empty() {
            return Err("O formato antigo do backup não suporta fotos.".to_string());
        }

        if !valid_key(&package.database_key) {
            return Err("A chave contida no backup é inválida.".to_string());
        }

        let database_hex = std::mem::take(&mut package.database_hex);

        let database = hex::decode(database_hex)
            .map_err(|_| "Os dados do banco no backup são inválidos.".to_string())?;

        if database.len() < 4096 || database.starts_with(b"SQLite format 3\0") {
            return Err("O backup não contém o banco criptografado esperado.".to_string());
        }

        let mut temporary = NamedTempFile::new_in(&data_dir)
            .map_err(|_| "Não foi possível preparar a restauração.".to_string())?;

        temporary
            .write_all(&database)
            .map_err(|_| "Não foi possível gravar o banco temporário.".to_string())?;

        temporary
            .as_file()
            .sync_all()
            .map_err(|_| "Não foi possível confirmar a gravação temporária.".to_string())?;

        Ok((temporary, package))
    })
    .await
    .map_err(|_| "A leitura do backup foi interrompida.".to_string())?
}

async fn replace_database(
    temporary: NamedTempFile,
    database_path: PathBuf,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let data_dir = database_path.parent().ok_or_else(|| {
            "A pasta do banco é inválida. Reinicie o aplicativo."
                .to_string()
        })?;

        let mut previous = tempfile::Builder::new()
            .prefix("before-restore-")
            .suffix(".db")
            .tempfile_in(data_dir)
            .map_err(|_| {
                "Não foi possível preservar o banco atual. Reinicie o aplicativo."
                    .to_string()
            })?;

        let mut original = File::open(&database_path).map_err(|_| {
            "Não foi possível ler o banco atual. Reinicie o aplicativo."
                .to_string()
        })?;

        std::io::copy(&mut original, previous.as_file_mut())
            .map_err(|_| {
                "Não foi possível copiar o banco atual. Reinicie o aplicativo."
                    .to_string()
            })?;

        previous.as_file().sync_all().map_err(|_| {
            "Não foi possível confirmar a cópia do banco anterior. Reinicie o aplicativo."
                .to_string()
        })?;

        drop(original);

        let (previous_file, previous_path) =
            previous.keep().map_err(|_| {
                "Não foi possível manter a cópia do banco anterior. Reinicie o aplicativo."
                    .to_string()
            })?;

        drop(previous_file);

        let installed = temporary
            .persist(&database_path)
            .map_err(|_| {
                "Não foi possível substituir o banco. A cópia anterior foi preservada. Reinicie o aplicativo."
                    .to_string()
            })?;

        installed.sync_all().map_err(|_| {
            "O banco foi substituído, mas houve erro ao confirmar a gravação. Reinicie o aplicativo."
                .to_string()
        })?;

        #[cfg(unix)]
        File::open(data_dir)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| {
                "O banco foi substituído, mas houve erro ao confirmar a pasta. Reinicie o aplicativo."
                    .to_string()
            })?;

        Ok(previous_path.to_string_lossy().into_owned())
    })
        .await
        .map_err(|_| {
            "A substituição do banco foi interrompida. Reinicie o aplicativo."
                .to_string()
        })?
}
