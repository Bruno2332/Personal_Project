use serde::{Deserialize, Serialize};

use crate::models::backup_photo::BackupPhoto;

#[derive(Serialize, Deserialize)]
pub struct BackupPhotoDto {
    pub file_path: String,
    pub image_hex: String,
}

impl From<BackupPhoto> for BackupPhotoDto {
    fn from(photo: BackupPhoto) -> Self {
        Self {
            file_path: photo.file_path,
            image_hex: photo.image_hex,
        }
    }
}

impl From<BackupPhotoDto> for BackupPhoto {
    fn from(dto: BackupPhotoDto) -> Self {
        Self {
            file_path: dto.file_path,
            image_hex: dto.image_hex,
        }
    }
}
