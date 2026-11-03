use crate::models::backup_photo::BackupPhoto;
use chrono::{DateTime, Utc};

pub struct BackupSettings {
    pub folder: String,
    pub password_account: String,
    pub last_automatic_backup: Option<DateTime<Utc>>,
}

pub struct BackupPackage {
    pub format_version: u32,
    pub application: String,
    pub created_at: String,
    pub database_key: String,
    pub database_hex: String,
    pub photos: Vec<BackupPhoto>,
}
