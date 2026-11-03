use std::path::Path;

use mockall::predicate::eq;

use crate::{
    dtos::physical_assessment_photos_dto::PhysicalAssessmentPhotosDto,
    errors::app_errors::AppError, models::physical_assessment_photos::PhotoPosition,
    repositories::physical_assessment_photos_repository::MockPhysicalAssessmentPhotosRepository,
    services::physical_assessment_photos_service,
    tests::factories::physical_assessment_factory::valid_photo,
};

fn valid_photo_dto() -> PhysicalAssessmentPhotosDto {
    PhysicalAssessmentPhotosDto::from(valid_photo())
}

fn is_not_found<T>(result: Result<T, AppError>, expected: &str) {
    assert!(matches!(result, Err(AppError::NotFound { message }) if message == expected));
}

fn is_database_error<T>(result: Result<T, AppError>, expected: &str) {
    assert!(matches!(result, Err(AppError::Database { message }) if message == expected));
}

#[tokio::test]
async fn find_all_maps_photo_rows_to_dtos() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_find_all_by_assessment_id()
        .with(eq(21))
        .times(1)
        .returning(|_| Ok(vec![valid_photo()]));

    let photos = physical_assessment_photos_service::find_all_by_assessment_id(&repository, 21)
        .await
        .unwrap();

    assert_eq!(photos.len(), 1);
    assert_eq!(photos[0].id, Some(61));
    assert_eq!(photos[0].position, PhotoPosition::Front);
}

#[tokio::test]
async fn find_all_maps_repository_error_to_database_error() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_find_all_by_assessment_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result =
        physical_assessment_photos_service::find_all_by_assessment_id(&repository, 21).await;

    is_database_error(result, "Erro ao buscar fotos da avaliação.");
}

#[tokio::test]
async fn find_by_id_returns_photo_dto() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_find_by_id()
        .with(eq(61))
        .times(1)
        .returning(|_| Ok(Some(valid_photo())));

    let photo = physical_assessment_photos_service::find_by_id(&repository, 61)
        .await
        .unwrap();

    assert_eq!(photo.id, Some(61));
    assert_eq!(photo.file_path, "assessment-photos/21/front.enc");
}

#[tokio::test]
async fn find_by_id_returns_not_found_when_photo_does_not_exist() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_find_by_id()
        .with(eq(404))
        .times(1)
        .returning(|_| Ok(None));

    let result = physical_assessment_photos_service::find_by_id(&repository, 404).await;

    is_not_found(result, "Foto não encontrada.");
}

#[tokio::test]
async fn find_by_id_maps_repository_error_to_database_error() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_find_by_id()
        .times(1)
        .returning(|_| Err(sqlx::Error::PoolClosed));

    let result = physical_assessment_photos_service::find_by_id(&repository, 61).await;

    is_database_error(result, "Erro ao buscar foto.");
}

#[tokio::test]
async fn read_content_returns_not_found_before_accessing_photo_files() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_find_by_id()
        .with(eq(404))
        .times(1)
        .returning(|_| Ok(None));

    let result = physical_assessment_photos_service::read_content(
        &repository,
        Path::new("/does/not/matter"),
        404,
    )
    .await;

    is_not_found(result, "Foto não encontrada.");
}

#[tokio::test]
async fn insert_rejects_invalid_assessment_id_before_file_or_repository_work() {
    let repository = MockPhysicalAssessmentPhotosRepository::new();
    let mut dto = valid_photo_dto();
    dto.physical_assessment_id = 0;

    let result =
        physical_assessment_photos_service::insert(&repository, Path::new("/does/not/matter"), dto)
            .await;

    assert!(matches!(
        result,
        Err(AppError::Validation { errors })
            if errors.len() == 1
                && errors[0].field == "physical_assessment_id"
                && errors[0].message == "Informe uma avaliação válida."
    ));
}

#[tokio::test]
async fn update_returns_updated_photo_dto() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_update()
        .withf(|id, position, description, _| {
            *id == 61
                && *position == PhotoPosition::Side
                && description.as_deref() == Some("Perfil")
        })
        .times(1)
        .returning(|_, position, description, _| {
            let mut photo = valid_photo();
            photo.position = position;
            photo.description = description;
            Ok(photo)
        });
    let mut dto = valid_photo_dto();
    dto.position = PhotoPosition::Side;
    dto.description = Some("Perfil".to_string());

    let updated = physical_assessment_photos_service::update(&repository, 61, dto)
        .await
        .unwrap();

    assert_eq!(updated.position, PhotoPosition::Side);
    assert_eq!(updated.description.as_deref(), Some("Perfil"));
}

#[tokio::test]
async fn update_maps_missing_row_to_not_found() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_update()
        .times(1)
        .returning(|_, _, _, _| Err(sqlx::Error::RowNotFound));

    let result =
        physical_assessment_photos_service::update(&repository, 404, valid_photo_dto()).await;

    is_not_found(result, "Foto não encontrada.");
}

#[tokio::test]
async fn delete_returns_not_found_without_touching_file_when_photo_is_missing() {
    let mut repository = MockPhysicalAssessmentPhotosRepository::new();
    repository
        .expect_find_by_id()
        .with(eq(404))
        .times(1)
        .returning(|_| Ok(None));

    let result = physical_assessment_photos_service::delete_by_id(
        &repository,
        Path::new("/does/not/matter"),
        404,
    )
    .await;

    is_not_found(result, "Foto não encontrada.");
}
