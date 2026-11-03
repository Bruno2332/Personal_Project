use crate::dtos::pagination::PageResponse;
use crate::dtos::student_dto::{StudentDto, StudentProjectionDto};
use crate::errors::app_errors::AppError;
use crate::services::student_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn find_all_students(
    pool: State<'_, SqlitePool>,
    page: i64,
    size: i64,
    name: Option<String>,
) -> Result<PageResponse<StudentProjectionDto>, AppError> {
    student_service::find_all(pool.inner(), page, size, name).await
}

#[tauri::command]
pub async fn find_student_by_id(
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<StudentDto, AppError> {
    student_service::find_by_id(pool.inner(), id).await
}

#[tauri::command]
pub async fn insert_student(
    pool: State<'_, SqlitePool>,
    dto: StudentDto,
) -> Result<StudentDto, AppError> {
    student_service::insert(pool.inner(), dto).await
}

#[tauri::command]
pub async fn update_student(
    pool: State<'_, SqlitePool>,
    id: i64,
    dto: StudentDto,
) -> Result<StudentDto, AppError> {
    student_service::update(pool.inner(), id, dto).await
}
