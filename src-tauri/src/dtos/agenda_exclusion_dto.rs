use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use crate::models::agenda_exclusions::AgendaExclusion;

#[derive(Debug, Deserialize)]
pub struct CreateAgendaExclusionDto {
    pub agenda_id: i64,
    pub occurrence_date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct AgendaExclusionDto {
    pub id: Option<i64>,
    pub agenda_id: i64,
    pub occurrence_date: NaiveDate,
    pub created_at: DateTime<Utc>,
}

impl From<CreateAgendaExclusionDto> for AgendaExclusion {
    fn from(dto: CreateAgendaExclusionDto) -> Self {
        Self {
            id: None,
            agenda_id: dto.agenda_id,
            occurrence_date: dto.occurrence_date,
            created_at: Utc::now(),
        }
    }
}

impl From<AgendaExclusion> for AgendaExclusionDto {
    fn from(entity: AgendaExclusion) -> Self {
        Self {
            id: entity.id,
            agenda_id: entity.agenda_id,
            occurrence_date: entity.occurrence_date,
            created_at: entity.created_at,
        }
    }
}