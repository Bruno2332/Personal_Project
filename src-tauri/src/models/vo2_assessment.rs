use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum PerceivedExertion {
    Light = 0,
    Moderate = 1,
    Intermediate = 2,
    Intense = 3,
    Maximal = 4,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Vo2Assessment {
    pub id: Option<i64>,
    pub physical_assessment_id: i64,
    pub treadmill_speed_kmh: Option<i32>,
    pub heart_rate_bpm: Option<i32>,
    pub perceived_exertion: Option<PerceivedExertion>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
