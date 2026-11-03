use crate::{
    dtos::agenda_dto::AgendaDto,
    models::agenda::{Agenda, AppointmentType, WeekDay},
    tests::factories::agenda_factory::{valid_agenda, valid_agenda_dto, valid_recurring_agenda},
};

#[test]
fn converts_agenda_model_to_dto_without_losing_fields() {
    let model = valid_recurring_agenda();

    let dto = AgendaDto::from(model);

    assert_eq!(dto.id, Some(12));
    assert_eq!(dto.student_id, 8);
    assert_eq!(dto.appointment_type, AppointmentType::Assessment);
    assert!(dto.is_recurring);
    assert_eq!(dto.week_day, Some(WeekDay::Monday));
    assert_eq!(dto.start_time.to_string(), "14:30:00");
    assert_eq!(dto.end_time.to_string(), "15:00:00");
    assert_eq!(dto.notes, "Avaliação recorrente");
    assert!(dto.scheduled_date.is_none());
    assert!(dto.recurrence_start_date.is_some());
    assert!(dto.recurrence_end_date.is_some());
}

#[test]
fn converts_agenda_dto_to_model_without_losing_fields() {
    let dto = valid_agenda_dto();

    let model = Agenda::from(dto);

    assert_eq!(model.id, None);
    assert_eq!(model.student_id, 7);
    assert_eq!(model.appointment_type, AppointmentType::Class);
    assert!(!model.is_recurring);
    assert_eq!(model.scheduled_date.unwrap().to_string(), "2026-11-02");
    assert!(model.week_day.is_none());
    assert_eq!(model.start_time.to_string(), "09:00:00");
    assert_eq!(model.end_time.to_string(), "10:00:00");
    assert!(model.recurrence_start_date.is_none());
    assert!(model.recurrence_end_date.is_none());
    assert_eq!(model.notes, "Aula de teste");
}

#[test]
fn dto_from_model_preserves_single_appointment_dates() {
    let dto = AgendaDto::from(valid_agenda());

    assert!(!dto.is_recurring);
    assert!(dto.scheduled_date.is_some());
    assert!(dto.week_day.is_none());
}
