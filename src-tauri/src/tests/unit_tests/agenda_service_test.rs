use chrono::NaiveDate;
use mockall::predicate::*;

use crate::{
    errors::app_errors::AppError,
    repositories::agenda_repository::MockAgendaRepository,
    services::agenda_service,
    tests::factories::agenda_factory::{valid_agenda, valid_agenda_dto},
};

fn assert_database_error<T>(result: Result<T, AppError>, expected_message: &str) {
    let Err(AppError::Database { message }) = result else {
        panic!("Esperava erro de banco.");
    };
    assert_eq!(message, expected_message);
}

#[tokio::test]
async fn find_by_period_returns_mapped_agenda_and_forwards_dates() {
    let start = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2026, 11, 7).unwrap();
    let mut repository = MockAgendaRepository::new();

    repository
        .expect_find_by_period()
        .with(eq(start), eq(end))
        .times(1)
        .returning(|_, _| Ok(vec![valid_agenda()]));

    let result = agenda_service::find_by_period(&repository, start, end)
        .await
        .unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, Some(11));
    assert_eq!(result[0].student_id, 7);
    assert_eq!(result[0].notes, "Aula de teste");
}

#[tokio::test]
async fn find_by_period_returns_empty_list_when_no_agenda_exists() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_find_by_period()
        .times(1)
        .returning(|_, _| Ok(Vec::new()));

    let result = agenda_service::find_by_period(
        &repository,
        NaiveDate::from_ymd_opt(2026, 11, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 11, 7).unwrap(),
    )
    .await
    .unwrap();

    assert!(result.is_empty());
}

#[tokio::test]
async fn find_by_period_maps_repository_error_to_database_error() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_find_by_period()
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    let result = agenda_service::find_by_period(
        &repository,
        NaiveDate::from_ymd_opt(2026, 11, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 11, 7).unwrap(),
    )
    .await;

    assert_database_error(result, "Erro ao buscar agendamento");
}

#[tokio::test]
async fn insert_returns_dto_from_created_agenda() {
    let mut repository = MockAgendaRepository::new();
    repository.expect_insert().times(1).returning(|entity| {
        assert_eq!(entity.id, None);
        assert_eq!(entity.student_id, 7);
        let mut created = entity;
        created.id = Some(11);
        Ok(created)
    });

    let result = agenda_service::insert(&repository, valid_agenda_dto())
        .await
        .unwrap();

    assert_eq!(result.id, Some(11));
    assert_eq!(result.student_id, 7);
    assert_eq!(result.notes, "Aula de teste");
}

#[tokio::test]
async fn insert_maps_repository_error_to_database_error() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_insert()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = agenda_service::insert(&repository, valid_agenda_dto()).await;

    assert_database_error(result, "Erro ao criar agendamento");
}

#[tokio::test]
async fn update_returns_dto_from_updated_agenda_and_forwards_id() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_update()
        .withf(|entity, id| entity.student_id == 7 && *id == 11)
        .times(1)
        .returning(|mut entity, id| {
            entity.id = Some(id);
            Ok(entity)
        });

    let result = agenda_service::update(&repository, 11, valid_agenda_dto())
        .await
        .unwrap();

    assert_eq!(result.id, Some(11));
    assert_eq!(result.student_id, 7);
}

#[tokio::test]
async fn update_returns_not_found_when_repository_reports_row_not_found() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_update()
        .with(always(), eq(404))
        .times(1)
        .returning(|_, _| Err(sqlx::Error::RowNotFound));

    let result = agenda_service::update(&repository, 404, valid_agenda_dto()).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Agendamento não encontrado"
    ));
}

#[tokio::test]
async fn update_maps_other_repository_errors_to_database_error() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_update()
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    let result = agenda_service::update(&repository, 11, valid_agenda_dto()).await;

    assert_database_error(result, "Erro ao atualizar agendamento");
}

#[tokio::test]
async fn delete_succeeds_when_repository_deletes_one_row() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_delete_by_id()
        .with(eq(11))
        .times(1)
        .returning(|_| Ok(1));

    agenda_service::delete(&repository, 11).await.unwrap();
}

#[tokio::test]
async fn delete_returns_not_found_when_repository_deletes_no_rows() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_delete_by_id()
        .with(eq(404))
        .times(1)
        .returning(|_| Ok(0));

    let result = agenda_service::delete(&repository, 404).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Agendamento não encontrado"
    ));
}

#[tokio::test]
async fn delete_maps_repository_error_to_database_error() {
    let mut repository = MockAgendaRepository::new();
    repository
        .expect_delete_by_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = agenda_service::delete(&repository, 11).await;

    assert_database_error(result, "Erro ao deletar agendamento");
}
