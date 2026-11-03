use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use sqlx::sqlite::SqlitePoolOptions;
use std::{fs, path::Path};
use uuid::Uuid;

use crate::{
    dtos::backup_settings_file_dto::BackupSettingsFileDto, models::backup::BackupSettings,
    services::backup_manager, tests::factories::backup_factory::valid_backup_settings,
};

fn settings_file_path(directory: &Path) -> std::path::PathBuf {
    let profile = if cfg!(debug_assertions) {
        "dev"
    } else {
        "release"
    };
    directory.join(format!("backup-settings-{profile}.json"))
}

fn write_settings(directory: &Path, settings: BackupSettings) {
    let dto = BackupSettingsFileDto::from(settings);
    fs::write(
        settings_file_path(directory),
        serde_json::to_vec(&dto).unwrap(),
    )
    .unwrap();
}

fn backup_filename(day: u32, id: u128) -> String {
    let profile = if cfg!(debug_assertions) {
        "dev"
    } else {
        "release"
    };
    let timestamp = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2026, 1, day).unwrap(),
        NaiveTime::from_hms_milli_opt(12, 0, 0, 0).unwrap(),
    );
    format!(
        "pt-backup-{profile}-{}_{}.ptbackup",
        timestamp.format("%Y%m%dT%H%M%S%3fZ"),
        Uuid::from_u128(id),
    )
}

#[test]
fn get_settings_returns_none_when_no_settings_file_exists() {
    let directory = tempfile::tempdir().unwrap();

    let settings = backup_manager::get_settings(directory.path()).unwrap();

    assert!(settings.is_none());
}

#[test]
fn get_settings_reads_the_configured_folder_and_last_automatic_date() {
    let directory = tempfile::tempdir().unwrap();
    let settings = valid_backup_settings();
    let expected_folder = settings.folder.clone();
    let expected_date = settings.last_automatic_backup;
    write_settings(directory.path(), settings);

    let loaded = backup_manager::get_settings(directory.path())
        .unwrap()
        .unwrap();

    assert_eq!(loaded.folder, expected_folder);
    assert_eq!(loaded.last_automatic_backup, expected_date);
}

#[test]
fn get_settings_reports_invalid_json() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(settings_file_path(directory.path()), b"not json").unwrap();

    let result = backup_manager::get_settings(directory.path());

    assert_eq!(
        result.err().unwrap(),
        "A configuração do backup está inválida."
    );
}

#[test]
fn remove_old_backups_keeps_five_newest_and_ignores_unrelated_files() {
    let directory = tempfile::tempdir().unwrap();
    let mut paths = Vec::new();

    for day in 1..=7 {
        let path = directory.path().join(backup_filename(day, day as u128));
        fs::write(&path, format!("backup {day}")).unwrap();
        paths.push(path);
    }

    fs::write(directory.path().join("notes.txt"), b"keep").unwrap();
    fs::write(directory.path().join("pt-backup-invalid.ptbackup"), b"keep").unwrap();

    backup_manager::remove_old_backups(directory.path(), paths.last().unwrap()).unwrap();

    assert!(!paths[0].exists());
    assert!(!paths[1].exists());
    assert!(paths[2..].iter().all(|path| path.exists()));
    assert!(directory.path().join("notes.txt").exists());
    assert!(directory.path().join("pt-backup-invalid.ptbackup").exists());
}

#[tokio::test]
async fn automatic_backup_is_skipped_when_last_run_was_less_than_seven_days_ago() {
    let directory = tempfile::tempdir().unwrap();
    let pool = SqlitePoolOptions::new()
        .connect_lazy("sqlite::memory:")
        .unwrap();

    let no_settings = backup_manager::create_automatic_if_due(&pool, directory.path())
        .await
        .unwrap();
    assert!(no_settings.is_none());

    let mut settings = valid_backup_settings();
    settings.folder = directory.path().to_string_lossy().into_owned();
    settings.last_automatic_backup = Some(Utc::now() - Duration::days(1));
    write_settings(directory.path(), settings);

    let result = backup_manager::create_automatic_if_due(&pool, directory.path())
        .await
        .unwrap();

    assert!(result.is_none());
}
