use age::{secrecy::SecretString, Encryptor};
use chrono::Utc;
use keyring::Entry;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
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

const MAX_PACKAGE_SIZE: usize = 256 * 1024 * 1024;

pub async fn create_backup<R: BackupRepository>(
    repository: &R,
    data_dir: &Path,
    destination: &Path,
    password: String,
) -> Result<(), String> {
    if password.chars().count() < 12 {
        return Err("Use uma senha de recuperação com pelo menos 12 caracteres.".to_string());
    }

    if destination.exists() {
        return Err("Já existe um arquivo nesse destino.".to_string());
    }

    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| "Selecione um caminho completo para salvar o backup.".to_string())?;

    if !parent.is_dir() {
        return Err("A pasta escolhida para o backup não existe.".to_string());
    }

    let database_key = run_file_task(|| {
        let filename = if cfg!(debug_assertions) {
            "personal-dev.db"
        } else {
            "personal.db"
        };

        let entry = Entry::new("personal_trainer_project.database", filename)
            .map_err(|_| "Não foi possível acessar o cofre do sistema.".to_string())?;

        let key = entry
            .get_password()
            .map_err(|_| "Não foi possível recuperar a chave do banco.".to_string())?;

        if key.len() != 64 || !key.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("A chave do banco possui formato inválido.".to_string());
        }

        Ok(key)
    })
    .await?;

    let temporary_directory = tempfile::tempdir()
        .map_err(|_| "Não foi possível criar a pasta temporária do backup.".to_string())?;

    let snapshot_path = temporary_directory.path().join("database.db");

    repository.create_snapshot(&snapshot_path).await?;

    repository
        .validate_database(&snapshot_path, &database_key)
        .await?;

    let photo_paths = repository
        .find_photo_paths(&snapshot_path, &database_key)
        .await?;

    let directory = data_dir.to_path_buf();
    let destination = destination.to_path_buf();
    let password = SecretString::from(password);

    run_file_task(move || {
        // Mantém o snapshot disponível até terminar a operação.
        let _temporary_directory = temporary_directory;

        let source = File::open(&snapshot_path)
            .map_err(|_| "Não foi possível abrir a cópia do banco.".to_string())?;

        let mut database_bytes = Vec::new();

        source
            .take((MAX_PACKAGE_SIZE / 2 + 1) as u64)
            .read_to_end(&mut database_bytes)
            .map_err(|_| "Não foi possível ler a cópia do banco.".to_string())?;

        if database_bytes.len() > MAX_PACKAGE_SIZE / 2 {
            return Err("O banco ultrapassa o limite do pacote de backup.".to_string());
        }

        if database_bytes.is_empty() || database_bytes.starts_with(b"SQLite format 3\0") {
            return Err("A cópia do banco não está no formato criptografado esperado.".to_string());
        }

        let database_hex = hex::encode(database_bytes);
        let mut content_size = database_hex.len();
        let mut photos = Vec::with_capacity(photo_paths.len());

        if !photo_paths.is_empty() {
            let photo_key = assessment_photo_key::load_key()?;

            for file_path in photo_paths {
                let bytes =
                    assessment_photo_files::decrypt_photo(&directory, &file_path, &photo_key)
                        .map_err(|error| {
                            format!("Não foi possível incluir a foto {file_path}: {error}")
                        })?;

                let photo_size = bytes
                    .len()
                    .checked_mul(2)
                    .ok_or_else(|| "O tamanho da foto excede o limite permitido.".to_string())?;

                content_size = content_size
                    .checked_add(photo_size)
                    .ok_or_else(|| "O backup ultrapassa o limite permitido.".to_string())?;

                if content_size > MAX_PACKAGE_SIZE {
                    return Err(
                        "Banco e fotos ultrapassam o limite de 256 MiB do pacote de backup."
                            .to_string(),
                    );
                }

                photos.push(BackupPhoto {
                    file_path,
                    image_hex: hex::encode(bytes),
                });
            }
        }

        let package = BackupPackage {
            format_version: 2,
            application: "personal_trainer_project".to_string(),
            created_at: Utc::now().to_rfc3339(),
            database_key,
            database_hex,
            photos,
        };

        let package_dto = BackupPackageDto::from(package);

        // O conteúdo descriptografado permanece somente na memória.
        let plaintext = serde_json::to_vec(&package_dto)
            .map_err(|_| "Não foi possível preparar o conteúdo do backup.".to_string())?;

        drop(package_dto);

        if plaintext.len() > MAX_PACKAGE_SIZE {
            return Err("O pacote de backup ultrapassa o limite de 256 MiB.".to_string());
        }

        let parent = destination
            .parent()
            .ok_or_else(|| "O destino do backup é inválido.".to_string())?;

        let mut temporary_file = NamedTempFile::new_in(parent)
            .map_err(|_| "Não foi possível criar o arquivo de backup.".to_string())?;

        let encryptor = Encryptor::with_user_passphrase(password);

        {
            let mut encrypted_writer = encryptor
                .wrap_output(temporary_file.as_file_mut())
                .map_err(|_| "Não foi possível iniciar a criptografia do backup.".to_string())?;

            encrypted_writer
                .write_all(&plaintext)
                .map_err(|_| "Não foi possível gravar os dados do backup.".to_string())?;

            encrypted_writer
                .finish()
                .map_err(|_| "Não foi possível finalizar a criptografia do backup.".to_string())?;
        }

        temporary_file
            .as_file()
            .sync_all()
            .map_err(|_| "Não foi possível confirmar a gravação do backup.".to_string())?;

        let saved_file = temporary_file
            .persist_noclobber(&destination)
            .map_err(|error| {
                if error.error.kind() == io::ErrorKind::AlreadyExists {
                    "Já existe um arquivo nesse destino.".to_string()
                } else {
                    "Não foi possível salvar o backup no destino escolhido.".to_string()
                }
            })?;

        drop(saved_file);

        Ok(())
    })
    .await
}
