use chrono::NaiveDate;
use sqlx::SqlitePool;
use tauri::State;

use crate::{
    dtos::agenda_exclusion_dto::{
        AgendaExclusionDto,
        CreateAgendaExclusionDto,
    },
    errors::app_errors::AppError,
    services::agenda_exclusion_service,
};

#[tauri::command]
pub async fn find_agenda_exclusions_by_period(
    pool: State<'_, SqlitePool>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AgendaExclusionDto>, AppError> {
    agenda_exclusion_service::find_by_period(
        pool.inner(),
        start_date,
        end_date,
    )
        .await
}

#[tauri::command]
pub async fn exclude_agenda_occurrence(
    pool: State<'_, SqlitePool>,
    dto: CreateAgendaExclusionDto,
) -> Result<AgendaExclusionDto, AppError> {
    agenda_exclusion_service::insert(pool.inner(), dto).await
}