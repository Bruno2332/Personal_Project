use chrono::{NaiveDate, NaiveTime, Utc};

use crate::{
    dtos::agenda_dto::AgendaDto,
    models::agenda::{Agenda, AppointmentType, WeekDay},
};

pub fn valid_agenda_dto() -> AgendaDto {
    let now = Utc::now();

    AgendaDto {
        id: None,
        student_id: 7,
        appointment_type: AppointmentType::Class,
        is_recurring: false,
        scheduled_date: Some(NaiveDate::from_ymd_opt(2026, 11, 2).unwrap()),
        week_day: None,
        start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        recurrence_start_date: None,
        recurrence_end_date: None,
        notes: "Aula de teste".to_string(),
        created_at: now,
        updated_at: now,
    }
}

pub fn valid_agenda() -> Agenda {
    let now = Utc::now();

    Agenda {
        id: Some(11),
        student_id: 7,
        appointment_type: AppointmentType::Class,
        is_recurring: false,
        scheduled_date: Some(NaiveDate::from_ymd_opt(2026, 11, 2).unwrap()),
        week_day: None,
        start_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        recurrence_start_date: None,
        recurrence_end_date: None,
        notes: "Aula de teste".to_string(),
        created_at: now,
        updated_at: now,
    }
}

pub fn valid_recurring_agenda() -> Agenda {
    let now = Utc::now();
    let start_date = NaiveDate::from_ymd_opt(2026, 11, 2).unwrap();

    Agenda {
        id: Some(12),
        student_id: 8,
        appointment_type: AppointmentType::Assessment,
        is_recurring: true,
        scheduled_date: None,
        week_day: Some(WeekDay::Monday),
        start_time: NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        end_time: NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
        recurrence_start_date: Some(start_date),
        recurrence_end_date: Some(NaiveDate::from_ymd_opt(2026, 12, 28).unwrap()),
        notes: "Avaliação recorrente".to_string(),
        created_at: now,
        updated_at: now,
    }
}
