use crate::{
    dtos::{
        backup_dto::BackupSettingsDto, backup_package_dto::BackupPackageDto,
        backup_settings_file_dto::BackupSettingsFileDto,
    },
    models::backup::BackupPackage,
    tests::factories::backup_factory::{valid_backup_package, valid_backup_settings},
};

#[test]
fn backup_package_dto_preserves_package_and_nested_photos() {
    let package = valid_backup_package();
    let dto = BackupPackageDto::from(package);

    assert_eq!(dto.format_version, 2);
    assert_eq!(dto.application, "personal_trainer_project");
    assert_eq!(dto.created_at, "2026-11-01T12:00:00+00:00");
    assert_eq!(dto.database_key, "ab".repeat(32));
    assert_eq!(dto.database_hex, "001122aabb");
    assert_eq!(dto.photos.len(), 1);
    assert_eq!(dto.photos[0].file_path, "assessment-photos/21/front.enc");
    assert_eq!(dto.photos[0].image_hex, "aabbcc");

    let restored = BackupPackage::from(dto);
    assert_eq!(restored.format_version, 2);
    assert_eq!(restored.application, "personal_trainer_project");
    assert_eq!(restored.database_hex, "001122aabb");
    assert_eq!(restored.photos.len(), 1);
    assert_eq!(restored.photos[0].image_hex, "aabbcc");
}

#[test]
fn backup_settings_file_dto_round_trips_sensitive_account_and_timestamp() {
    let settings = valid_backup_settings();
    let timestamp = settings.last_automatic_backup;
    let dto = BackupSettingsFileDto::from(&settings);

    assert_eq!(dto.folder, settings.folder);
    assert_eq!(dto.password_account, settings.password_account);
    assert_eq!(dto.last_automatic_backup, timestamp);

    let restored = crate::models::backup::BackupSettings::from(dto);
    assert_eq!(restored.folder, settings.folder);
    assert_eq!(restored.password_account, settings.password_account);
    assert_eq!(restored.last_automatic_backup, timestamp);
}

#[test]
fn backup_settings_response_dto_does_not_expose_password_account() {
    let settings = valid_backup_settings();
    let response = BackupSettingsDto::from(settings);

    assert_eq!(response.folder, "/tmp/test-backups");
    assert!(response.last_automatic_backup.is_some());
}
