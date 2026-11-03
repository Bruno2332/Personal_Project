use crate::dtos::financial_records_dto::{FinancialRecordsDto, FinancialRecordsProjectionDto};
use crate::dtos::pagination::PageResponse;
use crate::errors::app_errors::AppError;
use crate::services::financial_records_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn find_all_financial_records(
    pool: State<'_, SqlitePool>,
    page: i64,
    size: i64,
    name: Option<String>,
    monthly: Option<bool>,
    transaction_type: Option<crate::models::financial_records::TransactionType>,
    status: Option<String>,
    period: Option<String>,
    contract_id: Option<i64>,
) -> Result<PageResponse<FinancialRecordsProjectionDto>, AppError> {
    financial_records_service::find_all(
        pool.inner(),
        page,
        size,
        name,
        monthly,
        transaction_type,
        status,
        period,
        contract_id,
    )
    .await
}

#[tauri::command]
pub async fn insert_financial_record(
    pool: State<'_, SqlitePool>,
    dto: FinancialRecordsDto,
) -> Result<FinancialRecordsDto, AppError> {
    let mut connection = pool
        .inner()
        .acquire()
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao obter conexão com o banco.".to_string(),
        })?;
    financial_records_service::insert(&mut *connection, dto).await
}

#[tauri::command]
pub async fn update_financial_record(
    pool: State<'_, SqlitePool>,
    id: i64,
    dto: FinancialRecordsDto,
) -> Result<FinancialRecordsDto, AppError> {
    let mut connection = pool
        .inner()
        .acquire()
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao obter conexão com o banco.".to_string(),
        })?;
    financial_records_service::update(&mut *connection, id, dto).await
}

#[tauri::command]
pub async fn find_financial_summary(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<crate::dtos::financial_records_dto::FinancialMonthDto>, AppError> {
    financial_records_service::summary_months(pool.inner()).await
}

#[tauri::command]
pub async fn find_financial_contract_options(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<crate::dtos::financial_records_dto::FinancialContractOptionDto>, AppError> {
    financial_records_service::contract_options(pool.inner()).await
}

#[tauri::command]
pub async fn generate_monthly_charges(pool: State<'_, SqlitePool>) -> Result<u64, AppError> {
    let mut connection = pool.acquire().await.map_err(|_| AppError::Database {
        message: "Erro ao obter conexão".to_string(),
    })?;

    financial_records_service::generate_monthly_charges(&mut *connection).await
}
