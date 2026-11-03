use crate::models::agenda::{Agenda, AppointmentType, WeekDay};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgendaDto {
    pub id: Option<i64>,
    pub student_id: i64,
    pub appointment_type: AppointmentType,
    pub is_recurring: bool,
    pub scheduled_date: Option<NaiveDate>,
    pub week_day: Option<WeekDay>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub recurrence_start_date: Option<NaiveDate>,
    pub recurrence_end_date: Option<NaiveDate>,
    pub notes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Agenda> for AgendaDto {
    fn from(agenda: Agenda) -> Self {
        Self {
            id: agenda.id,
            student_id: agenda.student_id,
            appointment_type: agenda.appointment_type,
            is_recurring: agenda.is_recurring,
            scheduled_date: agenda.scheduled_date,
            week_day: agenda.week_day,
            start_time: agenda.start_time,
            end_time: agenda.end_time,
            recurrence_start_date: agenda.recurrence_start_date,
            recurrence_end_date: agenda.recurrence_end_date,
            notes: agenda.notes,
            created_at: agenda.created_at,
            updated_at: agenda.updated_at,
        }
    }
}

impl From<AgendaDto> for Agenda {
    fn from(dto: AgendaDto) -> Self {
        Self {
            id: dto.id,
            student_id: dto.student_id,
            appointment_type: dto.appointment_type,
            is_recurring: dto.is_recurring,
            scheduled_date: dto.scheduled_date,
            week_day: dto.week_day,
            start_time: dto.start_time,
            end_time: dto.end_time,
            recurrence_start_date: dto.recurrence_start_date,
            recurrence_end_date: dto.recurrence_end_date,
            notes: dto.notes,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}
