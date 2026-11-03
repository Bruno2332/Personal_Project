use crate::dtos::user_dto::UserDto;
use crate::errors::app_errors::AppError;
use crate::services::user_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn find_user_by_id(pool: State<'_, SqlitePool>, id: i64) -> Result<UserDto, AppError> {
    user_service::find_by_id(pool.inner(), id).await
}

#[tauri::command]
pub async fn exists_user(pool: State<'_, SqlitePool>) -> Result<bool, AppError> {
    user_service::exists(pool.inner()).await
}

#[tauri::command]
pub async fn insert_user(pool: State<'_, SqlitePool>, dto: UserDto) -> Result<UserDto, AppError> {
    user_service::insert(pool.inner(), dto).await
}

#[tauri::command]
pub async fn update_user(
    pool: State<'_, SqlitePool>,
    id: i64,
    dto: UserDto,
) -> Result<UserDto, AppError> {
    user_service::update(pool.inner(), id, dto).await
}
