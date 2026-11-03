use mockall::predicate::eq;

use crate::{
    errors::app_errors::AppError,
    repositories::user_repository::MockUserRepository,
    services::user_service,
    tests::factories::user_factory::{valid_user, valid_user_dto},
};

fn assert_database_error<T>(result: Result<T, AppError>, expected_message: &str) {
    let Err(AppError::Database { message }) = result else {
        panic!("Esperava erro de banco.");
    };
    assert_eq!(message, expected_message);
}

#[tokio::test]
async fn find_by_id_returns_dto_when_user_exists() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_find_by_id()
        .with(eq(7))
        .times(1)
        .returning(|_| Ok(Some(valid_user())));

    let result = user_service::find_by_id(&repository, 7).await.unwrap();

    assert_eq!(result.id, Some(7));
    assert_eq!(result.username, "Personal de teste");
    assert_eq!(result.email, "personal@example.com");
    assert_eq!(result.cref, "CREF-12345");
}

#[tokio::test]
async fn find_by_id_returns_not_found_when_user_does_not_exist() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_find_by_id()
        .with(eq(404))
        .times(1)
        .returning(|_| Ok(None));

    let result = user_service::find_by_id(&repository, 404).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Usuário não encontrado"
    ));
}

#[tokio::test]
async fn find_by_id_maps_repository_error_to_database_error() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_find_by_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = user_service::find_by_id(&repository, 7).await;

    assert_database_error(result, "Erro ao buscar usuário");
}

#[tokio::test]
async fn exists_returns_true_when_user_is_registered() {
    let mut repository = MockUserRepository::new();
    repository.expect_exists().times(1).returning(|| Ok(true));

    assert!(user_service::exists(&repository).await.unwrap());
}

#[tokio::test]
async fn exists_returns_false_when_no_user_is_registered() {
    let mut repository = MockUserRepository::new();
    repository.expect_exists().times(1).returning(|| Ok(false));

    assert!(!user_service::exists(&repository).await.unwrap());
}

#[tokio::test]
async fn exists_maps_repository_error_to_database_error() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_exists()
        .times(1)
        .returning(|| Err(sqlx::Error::PoolClosed));

    let result = user_service::exists(&repository).await;

    assert_database_error(result, "Erro ao verificar se existe usuário cadastrado");
}

#[tokio::test]
async fn insert_returns_dto_from_created_user() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_insert()
        .times(1)
        .returning(|mut received| {
            assert_eq!(received.username, "Personal de teste");
            assert_eq!(received.email, "personal@example.com");
            received.id = Some(1);
            Ok(received)
        });

    let result = user_service::insert(&repository, valid_user_dto())
        .await
        .unwrap();

    assert_eq!(result.id, Some(1));
    assert_eq!(result.username, "Personal de teste");
    assert_eq!(result.email, "personal@example.com");
}

#[tokio::test]
async fn insert_maps_repository_error_to_database_error() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_insert()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = user_service::insert(&repository, valid_user_dto()).await;

    assert_database_error(result, "Erro ao criar usuário");
}

#[tokio::test]
async fn update_returns_dto_from_updated_user() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_update()
        .withf(|received, id| received.username == "Personal de teste" && *id == 7)
        .times(1)
        .returning(|mut received, id| {
            received.id = Some(id);
            received.username = "Personal atualizado".into();
            Ok(received)
        });

    let result = user_service::update(&repository, 7, valid_user_dto())
        .await
        .unwrap();

    assert_eq!(result.id, Some(7));
    assert_eq!(result.username, "Personal atualizado");
}

#[tokio::test]
async fn update_returns_not_found_when_user_does_not_exist() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_update()
        .withf(|_, id| *id == 404)
        .times(1)
        .returning(|_, _| Err(sqlx::Error::RowNotFound));

    let result = user_service::update(&repository, 404, valid_user_dto()).await;

    assert!(matches!(
        result,
        Err(AppError::NotFound { message }) if message == "Usuário não encontrado"
    ));
}

#[tokio::test]
async fn update_maps_other_repository_errors_to_database_error() {
    let mut repository = MockUserRepository::new();
    repository
        .expect_update()
        .times(1)
        .returning(|_, _| Err(sqlx::Error::PoolClosed));

    let result = user_service::update(&repository, 7, valid_user_dto()).await;

    assert_database_error(result, "Erro ao atualizar usuário");
}
