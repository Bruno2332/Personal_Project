use crate::dtos::contract_dto::CreateContractRequest;
use crate::dtos::physical_assessment_dto::{PhysicalAssessmentCompleteDto, PhysicalAssessmentDto};
use crate::dtos::vo2_assessment_dto::Vo2AssessmentDto;
use crate::errors::app_errors::AppError;
use crate::services::physical_assessment_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn find_all_physical_assessments_by_student_id(
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<Vec<PhysicalAssessmentCompleteDto>, AppError> {
    physical_assessment_service::find_all_by_student_id(pool.inner(), id).await
}

#[tauri::command]
pub async fn insert_physical_assessment(
    pool: State<'_, SqlitePool>,
    contract_request: CreateContractRequest,
    dto: PhysicalAssessmentDto,
    vo2_assessments: Vec<Vo2AssessmentDto>,
) -> Result<PhysicalAssessmentDto, AppError> {
    let mut transaction = pool.inner().begin().await.map_err(|_| AppError::Database {
        message: "Erro ao iniciar transação".to_string(),
    })?;

    let created = physical_assessment_service::insert(
        &mut *transaction,
        contract_request,
        dto,
        vo2_assessments,
    )
    .await?;

    transaction.commit().await.map_err(|_| AppError::Database {
        message: "Erro ao confirmar criação do contrato".to_string(),
    })?;

    Ok(created)
}

#[tauri::command]
pub async fn update_physical_assessment(
    pool: State<'_, SqlitePool>,
    id: i64,
    dto: PhysicalAssessmentDto,
    vo2_assessments: Vec<Vo2AssessmentDto>,
    removed_vo2_ids: Vec<i64>,
) -> Result<PhysicalAssessmentDto, AppError> {
    let mut transaction = pool.inner().begin().await.map_err(|_| AppError::Database {
        message: "Erro ao iniciar transação".to_string(),
    })?;

    let updated = physical_assessment_service::update(
        &mut *transaction,
        id,
        dto,
        vo2_assessments,
        removed_vo2_ids,
    )
    .await?;

    transaction.commit().await.map_err(|_| AppError::Database {
        message: "Erro ao confirmar atualização da avaliação física".to_string(),
    })?;

    Ok(updated)
}

#[tauri::command]
pub async fn delete_physical_assessment(
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<(), AppError> {
    physical_assessment_service::delete(pool.inner(), id).await
}
