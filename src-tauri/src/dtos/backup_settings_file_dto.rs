use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::backup::BackupSettings;

#[derive(Serialize, Deserialize)]
pub struct BackupSettingsFileDto {
    pub folder: String,
    pub password_account: String,
    pub last_automatic_backup: Option<DateTime<Utc>>,
}

impl From<&BackupSettings> for BackupSettingsFileDto {
    fn from(model: &BackupSettings) -> Self {
        Self {
            folder: model.folder.clone(),
            password_account: model.password_account.clone(),
            last_automatic_backup: model.last_automatic_backup,
        }
    }
}

impl From<BackupSettings> for BackupSettingsFileDto {
    fn from(model: BackupSettings) -> Self {
        Self {
            folder: model.folder,
            password_account: model.password_account,
            last_automatic_backup: model.last_automatic_backup,
        }
    }
}

impl From<BackupSettingsFileDto> for BackupSettings {
    fn from(dto: BackupSettingsFileDto) -> Self {
        Self {
            folder: dto.folder,
            password_account: dto.password_account,
            last_automatic_backup: dto.last_automatic_backup,
        }
    }
}
