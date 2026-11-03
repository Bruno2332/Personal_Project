use crate::models::physical_assessment_photos::{PhotoPosition, PhysicalAssessmentPhotos};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalAssessmentPhotosDto {
    pub id: Option<i64>,
    pub physical_assessment_id: i64,
    pub file_path: String,
    pub position: PhotoPosition,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<PhysicalAssessmentPhotosDto> for PhysicalAssessmentPhotos {
    fn from(entity: PhysicalAssessmentPhotosDto) -> Self {
        Self {
            id: entity.id,
            physical_assessment_id: entity.physical_assessment_id,
            file_path: entity.file_path,
            position: entity.position,
            description: entity.description,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<PhysicalAssessmentPhotos> for PhysicalAssessmentPhotosDto {
    fn from(dto: PhysicalAssessmentPhotos) -> Self {
        Self {
            id: dto.id,
            physical_assessment_id: dto.physical_assessment_id,
            file_path: dto.file_path,
            position: dto.position,
            description: dto.description,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}
