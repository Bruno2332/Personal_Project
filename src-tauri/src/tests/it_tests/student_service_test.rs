use crate::{
    models::students::{Objective, Student},
    repositories::{
        contract_repository::ContractRepository, student_repository::StudentRepository,
    },
    services::student_service,
    tests::{
        factories::{contract_factory::valid_contract, student_factory::valid_student_dto},
        utils::database::create_test_database,
    },
};

#[tokio::test]
async fn insert_find_and_update_student_through_service_and_sqlite() {
    let pool = create_test_database().await;

    let mut dto = valid_student_dto();
    dto.name = "Bruno Teste Integração".into();
    dto.objective = Some(Objective::GeneralConditioningHealth);
    let created = student_service::insert(&pool, dto).await.unwrap();
    let id = created.id.expect("ID gerado pelo SQLite");
    assert_eq!(created.name, "Bruno Teste Integração");

    let loaded = student_service::find_by_id(&pool, id).await.unwrap();
    assert_eq!(loaded.cpf, "12345678909");

    let mut changed = loaded;
    changed.name = "Bruno Atualizado".into();
    changed.is_active = false;
    let updated = StudentRepository::update(&pool, id, Student::from(changed))
        .await
        .unwrap_or_else(|error| panic!("StudentRepository::update falhou: {error:?}"));
    assert_eq!(updated.name, "Bruno Atualizado");
    assert!(!updated.is_active);

    let fetched_again = student_service::find_by_id(&pool, id).await.unwrap();
    assert_eq!(fetched_again.name, "Bruno Atualizado");
    assert!(!fetched_again.is_active);
}

#[tokio::test]
async fn find_by_id_returns_not_found_for_missing_student() {
    let pool = create_test_database().await;
    let result = student_service::find_by_id(&pool, 987654).await;
    assert!(matches!(
        result,
        Err(crate::errors::app_errors::AppError::NotFound { .. })
    ));
}

#[tokio::test]
async fn student_projection_lists_student_once_with_its_training_contract() {
    let pool = create_test_database().await;
    let mut dto = valid_student_dto();
    dto.objective = Some(Objective::GeneralConditioningHealth);
    let student = StudentRepository::insert(&pool, Student::from(dto))
        .await
        .unwrap();
    let mut contract = valid_contract();
    contract.id = None;
    contract.student_id = student.id.unwrap();
    let mut connection = pool.acquire().await.unwrap();
    ContractRepository::insert(&mut *connection, contract)
        .await
        .unwrap();
    let mut second_contract = valid_contract();
    second_contract.id = None;
    second_contract.student_id = student.id.unwrap();
    second_contract.amount_cents = 22_000;
    ContractRepository::insert(&mut *connection, second_contract)
        .await
        .unwrap();
    drop(connection);
    let result = student_service::find_all(&pool, 0, 20, None).await.unwrap();

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].id, student.id);
    assert_eq!(result.items[0].amount_cents, 22_000);
    assert_eq!(result.total, 1);
}
