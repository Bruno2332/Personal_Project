use chrono::NaiveDate;

use crate::{
    models::students::Student,
    repositories::student_repository::StudentRepository,
    services::agenda_service,
    tests::{
        factories::{agenda_factory::valid_agenda_dto, student_factory::valid_student_dto},
        utils::database::create_test_database,
    },
};

#[tokio::test]
async fn agenda_service_persists_finds_updates_and_deletes_appointment() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(&pool, Student::from(valid_student_dto()))
        .await
        .unwrap();
    let mut dto = valid_agenda_dto();
    dto.student_id = student.id.unwrap();

    let created = agenda_service::insert(&pool, dto).await.unwrap();
    let id = created.id.unwrap();
    let date = NaiveDate::from_ymd_opt(2026, 11, 2).unwrap();
    let found = agenda_service::find_by_period(&pool, date, date)
        .await
        .unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].id, Some(id));

    let mut updated = valid_agenda_dto();
    updated.id = Some(id);
    updated.student_id = student.id.unwrap();
    updated.notes = "Horário atualizado".into();
    let updated = agenda_service::update(&pool, id, updated).await.unwrap();
    assert_eq!(updated.notes, "Horário atualizado");

    agenda_service::delete(&pool, id).await.unwrap();
    assert!(agenda_service::find_by_period(&pool, date, date)
        .await
        .unwrap()
        .is_empty());
}

#[tokio::test]
async fn recurring_agenda_is_returned_when_its_date_range_overlaps_query() {
    let pool = create_test_database().await;
    let student = StudentRepository::insert(&pool, Student::from(valid_student_dto()))
        .await
        .unwrap();
    let mut dto = crate::tests::factories::agenda_factory::valid_recurring_agenda();
    dto.id = None;
    dto.student_id = student.id.unwrap();
    let inserted = crate::repositories::agenda_repository::AgendaRepository::insert(&pool, dto)
        .await
        .unwrap();

    let found = agenda_service::find_by_period(
        &pool,
        NaiveDate::from_ymd_opt(2026, 12, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 12, 7).unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].id, inserted.id);
}
