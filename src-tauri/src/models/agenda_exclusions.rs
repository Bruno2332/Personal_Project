use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct AgendaExclusion {
    pub id: Option<i64>,
    pub agenda_id: i64,
    pub occurrence_date: NaiveDate,
    pub created_at: DateTime<Utc>,
}