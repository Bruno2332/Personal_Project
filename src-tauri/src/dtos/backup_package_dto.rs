use serde::{Deserialize, Serialize};

use crate::{dtos::backup_photo_dto::BackupPhotoDto, models::backup::BackupPackage};

#[derive(Serialize, Deserialize)]
pub struct BackupPackageDto {
    pub format_version: u32,
    pub application: String,
    pub created_at: String,
    pub database_key: String,
    pub database_hex: String,

    #[serde(default)]
    pub photos: Vec<BackupPhotoDto>,
}

impl From<BackupPackage> for BackupPackageDto {
    fn from(package: BackupPackage) -> Self {
        Self {
            format_version: package.format_version,
            application: package.application,
            created_at: package.created_at,
            database_key: package.database_key,
            database_hex: package.database_hex,
            photos: package
                .photos
                .into_iter()
                .map(BackupPhotoDto::from)
                .collect(),
        }
    }
}

impl From<BackupPackageDto> for BackupPackage {
    fn from(dto: BackupPackageDto) -> Self {
        Self {
            format_version: dto.format_version,
            application: dto.application,
            created_at: dto.created_at,
            database_key: dto.database_key,
            database_hex: dto.database_hex,
            photos: dto.photos.into_iter().map(Into::into).collect(),
        }
    }
}
