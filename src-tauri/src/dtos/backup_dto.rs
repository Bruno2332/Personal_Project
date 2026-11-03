use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::backup::BackupSettings;

#[derive(Deserialize)]
pub struct BackupSettingsRequestDto {
    pub folder: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BackupSettingsDto {
    pub folder: String,
    pub last_automatic_backup: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct BackupResultDto {
    pub path: String,
    pub warning: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RestoreBackupResponseDto {
    pub restored: bool,
    pub restart_required: bool,
    pub previous_database_path: Option<String>,
    pub error: Option<String>,
}

impl From<BackupSettings> for BackupSettingsDto {
    fn from(model: BackupSettings) -> Self {
        Self {
            folder: model.folder,
            last_automatic_backup: model.last_automatic_backup,
        }
    }
}
