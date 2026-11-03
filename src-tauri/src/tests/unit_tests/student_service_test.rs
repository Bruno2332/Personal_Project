use chrono::{NaiveDate, Utc};
use mockall::predicate::*;

use crate::{
    errors::app_errors::AppError,
    models::{
        contract::BillingType,
        students::{Gender, Objective, Student, StudentProjection},
    },
    repositories::student_repository::MockStudentRepository,
    services::student_service,
    tests::factories::student_factory::valid_student_dto,
};

fn student() -> Student {
    Student::from(valid_student_dto())
}

fn projection() -> StudentProjection {
    StudentProjection {
        id: Some(7),
        name: "Aluno de teste".to_string(),
        email: "aluno@example.com".to_string(),
        phone: "(11) 99999-9999".to_string(),
        objective: Objective::HypertrophyAndDefinition,
        is_active: true,
        first_due_date: Some(NaiveDate::from_ymd_opt(2026, 11, 10).unwrap()),
        amount_cents: 15000,
        billing_type: BillingType::Monthly,
    }
}

fn assert_database_error<T>(result: Result<T, AppError>, expected_message: &str) {
    let Err(AppError::Database { message }) = result else {
        panic!("Esperava erro de banco.");
    };
    assert_eq!(message, expected_message);
}

#[tokio::test]
async fn find_all_returns_mapped_page_and_forwards_filters() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_find_all_paginated()
        .withf(|page, size, name| *page == 2 && *size == 20 && name.as_deref() == Some("Ana"))
        .times(1)
        .returning(|_, _, _| Ok(vec![projection()]));
    repository
        .expect_count_all()
        .withf(|name| name.as_deref() == Some("Ana"))
        .times(1)
        .returning(|_| Ok(41));

    let result = student_service::find_all(&repository, 2, 20, Some("Ana".to_string()))
        .await
        .unwrap();

    assert_eq!(result.page, 2);
    assert_eq!(result.size, 20);
    assert_eq!(result.total, 41);
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].id, Some(7));
    assert_eq!(result.items[0].name, "Aluno de teste");
    assert_eq!(result.items[0].amount_cents, 15000);
    assert_eq!(result.items[0].billing_type, BillingType::Monthly);
}

#[tokio::test]
async fn find_all_forwards_empty_filter_as_none() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_find_all_paginated()
        .with(eq(0), eq(10), eq(None))
        .times(1)
        .returning(|_, _, _| Ok(Vec::new()));
    repository
        .expect_count_all()
        .with(eq(None))
        .times(1)
        .returning(|_| Ok(0));

    let result = student_service::find_all(&repository, 0, 10, None)
        .await
        .unwrap();

    assert!(result.items.is_empty());
    assert_eq!(result.total, 0);
}

#[tokio::test]
async fn find_all_maps_list_repository_error_to_database_error() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_find_all_paginated()
        .times(1)
        .returning(|_, _, _| Err(sqlx::Error::PoolClosed));

    let result = student_service::find_all(&repository, 0, 10, None).await;

    assert_database_error(result, "Erro ao buscar alunos");
}

#[tokio::test]
async fn find_all_maps_count_repository_error_to_database_error() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_find_all_paginated()
        .times(1)
        .returning(|_, _, _| Ok(Vec::new()));
    repository
        .expect_count_all()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = student_service::find_all(&repository, 0, 10, None).await;

    assert_database_error(result, "Erro ao contar alunos");
}

#[tokio::test]
async fn find_by_id_returns_student_dto_when_found() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_find_by_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Ok(Some(student())));

    let result = student_service::find_by_id(&repository, 7).await.unwrap();

    assert_eq!(result.name, "Aluno de teste");
    assert_eq!(result.cpf, "12345678909");
    assert_eq!(result.gender, Gender::Male);
    assert_eq!(result.email, "aluno@example.com");
}

#[tokio::test]
async fn find_by_id_returns_not_found_when_student_does_not_exist() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_find_by_id()
        .with(eq(404))
        .times(1)
        .returning(|_| Ok(None));

    let result = student_service::find_by_id(&repository, 404).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Aluno não encontrado"
    ));
}

#[tokio::test]
async fn find_by_id_maps_repository_error_to_database_error() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_find_by_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = student_service::find_by_id(&repository, 7).await;

    assert_database_error(result, "Erro ao buscar aluno");
}

#[tokio::test]
async fn insert_returns_dto_from_created_student() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_insert()
        .times(1)
        .returning(|mut received| {
            assert_eq!(received.name, "Aluno de teste");
            assert_eq!(received.id, None);
            received.id = Some(9);
            Ok(received)
        });

    let result = student_service::insert(&repository, valid_student_dto())
        .await
        .unwrap();

    assert_eq!(result.id, Some(9));
    assert_eq!(result.name, "Aluno de teste");
    assert_eq!(result.cpf, "12345678909");
}

#[tokio::test]
async fn insert_maps_repository_error_to_database_error() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_insert()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = student_service::insert(&repository, valid_student_dto()).await;

    assert_database_error(result, "Erro ao criar aluno");
}

#[tokio::test]
async fn update_returns_dto_from_updated_student() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_update()
        .withf(|id, received| *id == 7 && received.id.is_none())
        .times(1)
        .returning(|_, mut updated| {
            updated.id = Some(7);
            updated.updated_at = Utc::now();
            Ok(updated)
        });

    let result = student_service::update(&repository, 7, valid_student_dto())
        .await
        .unwrap();

    assert_eq!(result.id, Some(7));
    assert_eq!(result.name, "Aluno de teste");
}

#[tokio::test]
async fn update_returns_not_found_when_repository_reports_row_not_found() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_update()
        .with(eq(404), always())
        .times(1)
        .returning(|_, _| Err(sqlx::Error::RowNotFound));

    let result = student_service::update(&repository, 404, valid_student_dto()).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Aluno não encontrado"
    ));
}

#[tokio::test]
async fn update_maps_other_repository_errors_to_database_error() {
    let mut repository = MockStudentRepository::new();
    repository
        .expect_update()
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    let result = student_service::update(&repository, 7, valid_student_dto()).await;

    assert_database_error(result, "Erro ao atualizar aluno");
}
