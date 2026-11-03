use std::{fs, path::Path};

use crate::{repositories::backup_repository::MockBackupRepository, services::backup_service};

#[tokio::test]
async fn create_backup_rejects_short_recovery_password_before_repository_access() {
    let repository = MockBackupRepository::new();
    let result = backup_service::create_backup(
        &repository,
        Path::new("/tmp"),
        Path::new("/tmp/test.ptbackup"),
        "short".to_string(),
    )
    .await;

    assert_eq!(
        result.unwrap_err(),
        "Use uma senha de recuperação com pelo menos 12 caracteres."
    );
}

#[tokio::test]
async fn create_backup_refuses_to_overwrite_existing_destination() {
    let directory = tempfile::tempdir().unwrap();
    let destination = directory.path().join("existing.ptbackup");
    fs::write(&destination, b"keep existing file").unwrap();
    let repository = MockBackupRepository::new();

    let result = backup_service::create_backup(
        &repository,
        directory.path(),
        &destination,
        "long-enough-recovery-password".to_string(),
    )
    .await;

    assert_eq!(result.unwrap_err(), "Já existe um arquivo nesse destino.");
    assert_eq!(fs::read(destination).unwrap(), b"keep existing file");
}

#[tokio::test]
async fn create_backup_rejects_destination_in_missing_directory() {
    let directory = tempfile::tempdir().unwrap();
    let destination = directory.path().join("missing").join("backup.ptbackup");
    let repository = MockBackupRepository::new();

    let result = backup_service::create_backup(
        &repository,
        directory.path(),
        &destination,
        "long-enough-recovery-password".to_string(),
    )
    .await;

    assert_eq!(
        result.unwrap_err(),
        "A pasta escolhida para o backup não existe."
    );
}

#[tokio::test]
async fn create_backup_rejects_destination_without_parent_directory() {
    let repository = MockBackupRepository::new();

    let result = backup_service::create_backup(
        &repository,
        Path::new("/tmp"),
        Path::new("backup.ptbackup"),
        "long-enough-recovery-password".to_string(),
    )
    .await;

    assert_eq!(
        result.unwrap_err(),
        "Selecione um caminho completo para salvar o backup."
    );
}
