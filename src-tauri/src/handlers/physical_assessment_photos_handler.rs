use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, State};

use crate::{
    dtos::physical_assessment_photos_dto::PhysicalAssessmentPhotosDto,
    errors::app_errors::{AppError, FieldError},
    services::physical_assessment_photos_service,
};

#[tauri::command]
pub async fn find_all_physical_assessment_photos(
    pool: State<'_, SqlitePool>,
    assessment_id: i64,
) -> Result<Vec<PhysicalAssessmentPhotosDto>, AppError> {
    physical_assessment_photos_service::find_all_by_assessment_id(pool.inner(), assessment_id).await
}

#[tauri::command]
pub async fn find_physical_assessment_photo_by_id(
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<PhysicalAssessmentPhotosDto, AppError> {
    physical_assessment_photos_service::find_by_id(pool.inner(), id).await
}

#[tauri::command]
pub async fn insert_physical_assessment_photo(
    app: AppHandle,
    pool: State<'_, SqlitePool>,
    dto: PhysicalAssessmentPhotosDto,
) -> Result<PhysicalAssessmentPhotosDto, AppError> {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|_| AppError::Validation {
            errors: vec![FieldError {
                field: "file_path".to_string(),
                message: "Não foi possível localizar a pasta do aplicativo.".to_string(),
            }],
        })?;

    physical_assessment_photos_service::insert(pool.inner(), &data_dir, dto).await
}

#[tauri::command]
pub async fn update_physical_assessment_photo(
    pool: State<'_, SqlitePool>,
    id: i64,
    dto: PhysicalAssessmentPhotosDto,
) -> Result<PhysicalAssessmentPhotosDto, AppError> {
    physical_assessment_photos_service::update(pool.inner(), id, dto).await
}

#[tauri::command]
pub async fn delete_physical_assessment_photo(
    app: AppHandle,
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<(), AppError> {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|_| AppError::Validation {
            errors: vec![FieldError {
                field: "file_path".to_string(),
                message: "Não foi possível localizar a pasta do aplicativo.".to_string(),
            }],
        })?;

    physical_assessment_photos_service::delete_by_id(pool.inner(), &data_dir, id).await
}

#[tauri::command]
pub async fn read_physical_assessment_photo(
    app: AppHandle,
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<tauri::ipc::Response, AppError> {
    let directory = app
        .path()
        .app_local_data_dir()
        .map_err(|_| AppError::Validation {
            errors: vec![FieldError {
                field: "file_path".to_string(),
                message: "Não foi possível localizar a pasta do aplicativo.".to_string(),
            }],
        })?;

    let bytes =
        physical_assessment_photos_service::read_content(pool.inner(), &directory, id).await?;

    Ok(tauri::ipc::Response::new(bytes))
}
