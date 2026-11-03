use crate::dtos::contract_dto::{ContractDto, CreateContractRequest};
use crate::errors::app_errors::AppError;
use crate::services::contract_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn find_all_contracts_by_student_id(
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<Vec<ContractDto>, AppError> {
    let mut connection = pool.acquire().await.map_err(|_| AppError::Database {
        message: "Erro ao obter conexão".to_string(),
    })?;

    contract_service::find_all_by_student_id(&mut *connection, id).await
}

#[tauri::command]
pub async fn insert_contract(
    pool: State<'_, SqlitePool>,
    dto: CreateContractRequest,
) -> Result<ContractDto, AppError> {
    let mut transaction = pool.inner().begin().await.map_err(|_| AppError::Database {
        message: "Erro ao iniciar transação".to_string(),
    })?;

    let created = contract_service::insert(&mut *transaction, dto).await?;

    transaction.commit().await.map_err(|_| AppError::Database {
        message: "Erro ao confirmar criação do contrato".to_string(),
    })?;

    Ok(created)
}

#[tauri::command]
pub async fn update_contract(
    pool: State<'_, SqlitePool>,
    id: i64,
    dto: CreateContractRequest,
) -> Result<ContractDto, AppError> {
    let mut transaction = pool.inner().begin().await.map_err(|_| AppError::Database {
        message: "Erro ao iniciar transação".to_string(),
    })?;

    let updated = contract_service::update(&mut *transaction, id, dto).await?;

    transaction.commit().await.map_err(|_| AppError::Database {
        message: "Erro ao confirmar criação do contrato".to_string(),
    })?;

    Ok(updated)
}

pub async fn complete_expired_contracts(
    pool: &SqlitePool,
) -> Result<u64, AppError> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao iniciar encerramento dos contratos."
                .to_string(),
        })?;

    let completed = contract_service::complete_expired_contracts(
        &mut *transaction,
    )
        .await?;

    transaction
        .commit()
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao confirmar encerramento dos contratos."
                .to_string(),
        })?;

    Ok(completed)
}
