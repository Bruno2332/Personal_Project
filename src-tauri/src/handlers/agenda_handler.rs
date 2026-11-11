use crate::dtos::agenda_dto::AgendaDto;
use crate::errors::app_errors::AppError;
use crate::services::agenda_service;
use chrono::NaiveDate;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn find_agenda_by_period(
    pool: State<'_, SqlitePool>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AgendaDto>, AppError> {
    agenda_service::find_by_period(pool.inner(), start_date, end_date).await
}

#[tauri::command]
pub async fn end_agenda_recurrence(
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<AgendaDto, AppError> {
    agenda_service::end_recurrence(pool.inner(), id).await
}

#[tauri::command]
pub async fn insert_agenda(
    pool: State<'_, SqlitePool>,
    dto: AgendaDto,
) -> Result<AgendaDto, AppError> {
    agenda_service::insert(pool.inner(), dto).await
}

#[tauri::command]
pub async fn update_agenda(
    pool: State<'_, SqlitePool>,
    id: i64,
    dto: AgendaDto,
) -> Result<AgendaDto, AppError> {
    agenda_service::update(pool.inner(), id, dto).await
}

#[tauri::command]
pub async fn delete_agenda(pool: State<'_, SqlitePool>, id: i64) -> Result<(), AppError> {
    agenda_service::delete(pool.inner(), id).await
}
