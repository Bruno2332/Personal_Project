use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum PhotoPosition {
    Front = 0,
    Side = 1,
    Back = 2,
}

#[derive(Debug, FromRow)]
pub struct PhysicalAssessmentPhotos {
    pub id: Option<i64>,
    pub physical_assessment_id: i64,
    pub file_path: String,
    pub position: PhotoPosition,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
