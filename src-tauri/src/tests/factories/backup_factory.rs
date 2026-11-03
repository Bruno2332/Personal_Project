use chrono::Utc;

use crate::models::{
    backup::{BackupPackage, BackupSettings},
    backup_photo::BackupPhoto,
};

pub fn valid_backup_package() -> BackupPackage {
    BackupPackage {
        format_version: 2,
        application: "personal_trainer_project".to_string(),
        created_at: "2026-11-01T12:00:00+00:00".to_string(),
        database_key: "ab".repeat(32),
        database_hex: "001122aabb".to_string(),
        photos: vec![BackupPhoto {
            file_path: "assessment-photos/21/front.enc".to_string(),
            image_hex: "aabbcc".to_string(),
        }],
    }
}

pub fn valid_backup_settings() -> BackupSettings {
    BackupSettings {
        folder: "/tmp/test-backups".to_string(),
        password_account: "dev-test-account".to_string(),
        last_automatic_backup: Some(Utc::now()),
    }
}
