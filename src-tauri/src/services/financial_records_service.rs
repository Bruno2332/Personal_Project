use chrono::Utc;
use crate::dtos::financial_records_dto::{FinancialRecordsDto, FinancialRecordsProjectionDto};
use crate::dtos::pagination::PageResponse;
use crate::errors::app_errors::{AppError, FieldError};
use crate::models::financial_records::{FinancialRecords, TransactionStatus};
use crate::repositories::financial_records_repository::{
    FinancialRecordsConnectionRepository, FinancialRecordsPoolRepository,
};
use crate::utils::validations::validate_dto;

pub async fn find_all<R: FinancialRecordsPoolRepository>(
    repository: &R,
    page: i64,
    size: i64,
    name: Option<String>,
    monthly: Option<bool>,
    transaction_type: Option<crate::models::financial_records::TransactionType>,
    status: Option<String>,
    period: Option<String>,
    contract_id: Option<i64>,
) -> Result<PageResponse<FinancialRecordsProjectionDto>, AppError> {
    if page < 0 || !(1..=500).contains(&size) || page.checked_mul(size).is_none() {
        return Err(AppError::Validation {
            errors: vec![FieldError {
                field: "page".into(),
                message: "Paginação inválida.".into(),
            }],
        });
    }
    if status
        .as_deref()
        .is_some_and(|value| !["Pending", "Paid", "Cancelled", "Overdue"].contains(&value))
    {
        return Err(AppError::Validation {
            errors: vec![FieldError {
                field: "status".into(),
                message: "Status inválido.".into(),
            }],
        });
    }
    if period.as_deref().is_some_and(|period| {
        !matches!(period, "1m" | "3m" | "6m" | "12m")
    }) {
        return Err(AppError::Validation {
            errors: vec![FieldError {
                field: "period".into(),
                message: "Período inválido.".into(),
            }],
        });
    }
    let financial_records = repository
        .find_all_paginated(
            page,
            size,
            name.clone(),
            monthly,
            transaction_type,
            status.clone(),
            period.clone(),
            contract_id,
        )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar registro financeiros".to_string(),
        })?;

    let total = repository
        .count_all(name, monthly, transaction_type, status, period, contract_id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao contar registros financeiros".to_string(),
        })?;

    let items = financial_records
        .into_iter()
        .map(FinancialRecordsProjectionDto::from)
        .collect();

    Ok(PageResponse {
        items,
        page,
        size,
        total,
    })
}

pub async fn insert<R: FinancialRecordsConnectionRepository>(
    repository: &mut R,
    dto: FinancialRecordsDto,
) -> Result<FinancialRecordsDto, AppError> {
    validate_dto(&dto)?;
    
    let mut result = FinancialRecords::from(dto);
    let now = Utc::now();
    result.created_at = now;
    result.updated_at = now;

    let created = repository
        .insert(result)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao criar registro financeiro".to_string(),
        })?;

    Ok(FinancialRecordsDto::from(created))
}

pub async fn update<R: FinancialRecordsConnectionRepository>(
    repository: &mut R,
    id: i64,
    dto: FinancialRecordsDto,
) -> Result<FinancialRecordsDto, AppError> {

    validate_dto(&dto)?;
    
    let mut result = FinancialRecords::from(dto);
    result.updated_at = Utc::now();

    let updated = repository
        .update(id, result)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message: "Registro financeiro não encontrado".to_string(),
            },
            _ => AppError::Database {
                message: "Erro ao atualizar registro financeiro".to_string(),
            },
        })?;

    Ok(FinancialRecordsDto::from(updated))
}

pub async fn summary_months<R: FinancialRecordsPoolRepository>(
    repository: &R,
) -> Result<Vec<crate::dtos::financial_records_dto::FinancialMonthDto>, AppError> {
    repository.summary_months().await.map_err(|error| {
        eprintln!("Financial summary: {error:?}");
        AppError::Database {
            message: "Erro ao buscar resumo financeiro".into(),
        }
    })
}

pub async fn contract_options<R: FinancialRecordsPoolRepository>(
    repository: &R,
) -> Result<Vec<crate::dtos::financial_records_dto::FinancialContractOptionDto>, AppError> {
    repository.contract_options().await.map_err(|error| {
        eprintln!("Financial contract options: {error:?}");
        AppError::Database {
            message: "Erro ao buscar opções de contrato".into(),
        }
    })
}

pub async fn sync_contract_records<R: FinancialRecordsConnectionRepository>(
    repository: &mut R,
    contract_id: i64,
    records: Vec<FinancialRecordsDto>,
) -> Result<(), AppError> {
    let mut existing = repository
        .find_all_by_contract_id(contract_id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar registros financeiros do contrato.".to_string(),
        })?;

    let protected_record_error = || AppError::Validation {
        errors: vec![FieldError {
            field: "financial_records".to_string(),
            message: "A alteração modifica ou remove uma parcela já paga ou cancelada.".to_string(),
        }],
    };

    for mut record in records {
        record.contract_id = Some(contract_id);

        let position = existing
            .iter()
            .position(|current| current.installment_number == record.installment_number);

        if let Some(position) = position {
            let current = existing.remove(position);

            // Preserva parcelas pagas ou canceladas.
            if current.status_transaction != TransactionStatus::Pending {
                if current.amount_cent != record.amount_cent
                    || current.due_date != record.due_date
                    || current.payment_method != record.payment_method
                {
                    return Err(protected_record_error());
                }

                continue;
            }

            let record_id = current.id.ok_or_else(|| AppError::Database {
                message: "Registro financeiro encontrado sem ID.".to_string(),
            })?;

            record.id = Some(record_id);
            record.created_at = current.created_at;

            // Editar o contrato não redefine o pagamento de cada parcela.
            record.status_transaction = current.status_transaction;
            record.payment_date = current.payment_date;

            update(&mut *repository, record_id, record).await?;
        } else {
            // Essa parcela ainda não existe.
            record.id = None;
            insert(&mut *repository, record).await?;
        }
    }

    // As parcelas restantes não fazem mais parte do novo cronograma.
    for current in existing {
        if current.status_transaction != TransactionStatus::Pending {
            return Err(protected_record_error());
        }

        let record_id = current.id.ok_or_else(|| AppError::Database {
            message: "Registro financeiro encontrado sem ID.".to_string(),
        })?;

        repository
            .delete_pending_by_id(record_id)
            .await
            .map_err(|_| AppError::Database {
                message: "Erro ao remover parcela pendente.".to_string(),
            })?;
    }

    Ok(())
}

pub async fn generate_monthly_charges<R: FinancialRecordsConnectionRepository>(
    repository: &mut R,
) -> Result<u64, AppError> {
    let today = chrono::Local::now().date_naive();

    let inserted = repository
        .generate_monthly_charges(today)
        .await
        .map_err(|error| {
            eprintln!("Erro ao gerar mensalidades: {error:?}");

            AppError::Database {
                message: "Erro ao gerar mensalidades".to_string(),
            }
        })?;

    Ok(inserted)
}
