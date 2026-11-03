use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::path::Path;

use crate::{
    repositories::backup_repository::BackupRepository, services::restore_service,
    tests::utils::database::create_test_database,
};

async fn create_encrypted_database(path: &Path, key: &str) -> SqlitePool {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .pragma("key", format!("\"x'{key}'\""))
        .foreign_keys(true)
        .with_regexp();
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    sqlx::migrate!("db/migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn restore_preparation_validates_migrations_and_rekeys_real_encrypted_sqlite_file() {
    let directory = tempfile::tempdir().unwrap();
    let database_path = directory.path().join("backup.db");
    let imported_key = "11".repeat(32);
    let local_key = "22".repeat(32);
    let pool = create_encrypted_database(&database_path, &imported_key).await;
    drop(pool);

    let repository = create_test_database().await;

    restore_service::prepare_restored_database(
        &repository,
        &database_path,
        &imported_key,
        &local_key,
    )
    .await
    .unwrap();

    BackupRepository::validate_database(&repository, &database_path, &local_key)
        .await
        .unwrap();
    assert!(
        BackupRepository::validate_database(&repository, &database_path, &imported_key)
            .await
            .is_err()
    );

    let count = BackupRepository::count_required_tables(&repository, &database_path, &local_key)
        .await
        .unwrap();
    assert_eq!(count, 8);
}
