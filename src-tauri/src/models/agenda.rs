use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum AppointmentType {
    Class = 0,
    Assessment = 1,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum WeekDay {
    Sunday = 0,
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Agenda {
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
