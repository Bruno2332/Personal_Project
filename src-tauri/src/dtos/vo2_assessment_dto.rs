use crate::domain::fc_classification::ExerciseHeartRateClassification;
use crate::models::vo2_assessment::{PerceivedExertion, Vo2Assessment};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct Vo2AssessmentDto {
    pub id: Option<i64>,
    pub physical_assessment_id: i64,
    #[validate(range(min = 4, max = 25, message = "A velocidade deve estar entre 4 e 25 km/h."))]
    pub treadmill_speed_kmh: Option<i32>,
    #[validate(range(min = 1, max = 250, message = "A frequência cardíaca deve estar entre 1 e 250 bpm."))]
    pub heart_rate_bpm: Option<i32>,
    pub perceived_exertion: Option<PerceivedExertion>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Vo2Assessment> for Vo2AssessmentDto {
    fn from(vo2: Vo2Assessment) -> Self {
        Self {
            id: vo2.id,
            physical_assessment_id: vo2.physical_assessment_id,
            treadmill_speed_kmh: vo2.treadmill_speed_kmh,
            heart_rate_bpm: vo2.heart_rate_bpm,
            perceived_exertion: vo2.perceived_exertion,
            created_at: vo2.created_at,
            updated_at: vo2.updated_at,
        }
    }
}

impl From<Vo2AssessmentDto> for Vo2Assessment {
    fn from(dto: Vo2AssessmentDto) -> Self {
        Self {
            id: dto.id,
            physical_assessment_id: dto.physical_assessment_id,
            treadmill_speed_kmh: dto.treadmill_speed_kmh,
            heart_rate_bpm: dto.heart_rate_bpm,
            perceived_exertion: dto.perceived_exertion,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vo2AssessmentCompleteDto {
    pub vo2_assessment_dto: Vo2AssessmentDto,
    pub fc_heart_percentage: Option<f64>,
    pub fc_heart_classification: Option<ExerciseHeartRateClassification>,
}
