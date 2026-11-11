use crate::dtos::contract_dto::{ContractDto, CreateContractRequest};
use crate::dtos::financial_records_dto::FinancialRecordsDto;
use crate::errors::app_errors::{AppError, FieldError};
use crate::models::contract::{BillingType, Contract, ContractStatus, ServiceType};
use crate::models::financial_records::{PaymentMethod, TransactionType};
use crate::repositories::contract_repository::ContractRepository;
use crate::repositories::financial_records_repository::FinancialRecordsConnectionRepository;
use crate::repositories::student_repository::StudentStatusRepository;
use crate::services::financial_records_service;
use chrono::{Datelike, Months, NaiveDate, Utc};
use crate::utils::validations::validate_dto;

pub async fn find_all_by_student_id<R: ContractRepository>(
    repository: &mut R,
    id: i64,
) -> Result<Vec<ContractDto>, AppError> {
    let contracts =
        repository
            .find_all_by_student_id(id)
            .await
            .map_err(|_| AppError::Database {
                message: "Erro ao buscar contratos".to_string(),
            })?;

    let items = contracts.into_iter().map(ContractDto::from).collect();

    Ok(items)
}

pub async fn insert<R>(
    repository: &mut R,
    dto: CreateContractRequest,
) -> Result<ContractDto, AppError>
where
    R: ContractRepository + FinancialRecordsConnectionRepository + StudentStatusRepository,
{
    let mut result = apply_contract_duration(
        Contract::from(dto.contract_dto),
    )?;
    let now = Utc::now();
    result.created_at = now;
    result.updated_at = now;

    let created = ContractRepository::insert(&mut *repository, result)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao criar contrato".to_string(),
        })?;

    let records = build_financial_records(&created, &dto.financial_record)?;

    for record in records {
        financial_records_service::insert(&mut *repository, record).await?;
    }

    sync_student_is_active(&mut *repository, created.student_id).await?;

    Ok(ContractDto::from(created))
}

pub async fn update<R>(
    repository: &mut R,
    id: i64,
    dto: CreateContractRequest,
) -> Result<ContractDto, AppError>
where
    R: ContractRepository
    + FinancialRecordsConnectionRepository
    + StudentStatusRepository,
{
    validate_dto(&dto.contract_dto)?;

    if dto.contract_dto.billing_type == BillingType::Package {
        validate_dto(&dto.financial_record)?;
    }

    let mut result = apply_contract_duration(
        Contract::from(dto.contract_dto),
    )?;

    result.updated_at = Utc::now();

    let updated = ContractRepository::update(
        &mut *repository,
        result,
        id,
    )
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound {
                message: "Contrato não encontrado".to_string(),
            },
            _ => AppError::Database {
                message: "Erro ao atualizar contrato".to_string(),
            },
        })?;

    if updated.billing_type == BillingType::Package {
        let records = build_financial_records(
            &updated,
            &dto.financial_record,
        )?;

        financial_records_service::sync_contract_records(
            &mut *repository,
            id,
            records,
        )
            .await?;
    }

    sync_student_is_active(
        &mut *repository,
        updated.student_id,
    )
        .await?;

    Ok(ContractDto::from(updated))
}

async fn sync_student_is_active<R>(repository: &mut R, student_id: i64) -> Result<(), AppError>
where
    R: ContractRepository + StudentStatusRepository,
{
    let contracts = ContractRepository::find_all_by_student_id(&mut *repository, student_id)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao buscar contratos do aluno".to_string(),
        })?;

    let is_active = contracts
        .iter()
        .any(|contract| contract.contract_status == ContractStatus::Active);

    StudentStatusRepository::update_is_active(&mut *repository, student_id, is_active)
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao atualizar status do aluno".to_string(),
        })?;

    Ok(())
}

pub async fn complete_expired_contracts<R>(
    repository: &mut R,
) -> Result<u64, AppError>
where
    R: ContractRepository + StudentStatusRepository,
{
    let today = chrono::Local::now().date_naive();

    let student_ids = ContractRepository::complete_expired(
        &mut *repository,
        today,
        Utc::now(),
    )
        .await
        .map_err(|_| AppError::Database {
            message: "Erro ao encerrar contratos vencidos.".to_string(),
        })?;

    let completed = student_ids.len() as u64;

    let unique_students: std::collections::HashSet<i64> =
        student_ids.into_iter().collect();

    for student_id in unique_students {
        sync_student_is_active(
            &mut *repository,
            student_id,
        )
            .await?;
    }

    Ok(completed)
}

fn build_financial_records(
    contract: &Contract,
    financial: &FinancialRecordsDto,
) -> Result<Vec<FinancialRecordsDto>, AppError> {
    let validation_error = |field: &str, message: &str| AppError::Validation {
        errors: vec![FieldError {
            field: field.to_string(),
            message: message.to_string(),
        }],
    };

    let contract_id = contract
        .id
        .ok_or_else(|| validation_error("contract_id", "O contrato precisa estar salvo."))?;

    let installments = match financial.payment_method {
        PaymentMethod::Pix => contract.installment_count,
        PaymentMethod::CreditCard => 1,
    };

    if installments < 1 {
        return Err(validation_error(
            "installment_count",
            "A quantidade de parcelas deve ser maior que zero.",
        ));
    }

    let value = contract.amount_cents / installments;
    let remaining_cents = contract.amount_cents % installments;

    let first_due_date = financial.due_date;
    let due_day = contract.due_day.unwrap_or(first_due_date.day() as i32) as u32;

    let now = Utc::now();
    let mut records = Vec::with_capacity(installments as usize);

    for index in 0..installments {
        let due_date =
            due_date_for_installment(first_due_date, due_day, index as u32).ok_or_else(|| {
                validation_error(
                    "due_date",
                    "Não foi possível calcular o vencimento da parcela.",
                )
            })?;

        let amount_cent = value + if index < remaining_cents { 1 } else { 0 };

        records.push(FinancialRecordsDto {
            id: None,
            contract_id: Some(contract_id),
            transaction_type: TransactionType::Income,
            description: contract.contract_description.clone(),
            amount_cent,
            due_date,
            payment_date: financial.payment_date,
            payment_method: financial.payment_method,
            installment_number: index + 1,
            status_transaction: financial.status_transaction,
            created_at: now,
            updated_at: now,
        });
    }

    Ok(records)
}

fn due_date_for_installment(
    first_due_date: NaiveDate,
    due_day: u32,
    index: u32,
) -> Option<NaiveDate> {
    if index == 0 {
        return Some(first_due_date);
    }

    first_due_date
        .checked_add_months(Months::new(index))
        .and_then(|date| date.with_day(due_day))
}

fn apply_contract_duration(
    mut contract: Contract,
) -> Result<Contract, AppError> {
    if contract.service_type != ServiceType::Training
        || contract.billing_type != BillingType::Package
    {
        return Ok(contract);
    }

    let months = contract
        .duration_months
        .filter(|months| (1..=99).contains(months))
        .ok_or_else(|| AppError::Validation {
            errors: vec![FieldError {
                field: "duration_months".to_string(),
                message: "Informe a duração do pacote entre 1 e 99 meses."
                    .to_string(),
            }],
        })?;

    let end_date = contract
        .start_date
        .checked_add_months(Months::new(months as u32))
        .and_then(|date| date.pred_opt())
        .ok_or_else(|| AppError::Validation {
            errors: vec![FieldError {
                field: "duration_months".to_string(),
                message: "Não foi possível calcular o término do contrato."
                    .to_string(),
            }],
        })?;

    contract.end_date = Some(end_date);

    if contract.contract_status == ContractStatus::Active
        && end_date < chrono::Local::now().date_naive()
    {
        contract.contract_status = ContractStatus::Completed;
    }

    Ok(contract)
}