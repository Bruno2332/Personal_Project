use crate::models::financial_records::{
    FinancialRecords, FinancialRecordsProjection, PaymentMethod, TransactionStatus, TransactionType,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FinancialRecordsDto {
    pub id: Option<i64>,
    pub contract_id: Option<i64>,
    pub transaction_type: TransactionType,
    pub description: String,
    #[validate(range(
        min = 1,
        message = "O valor do lançamento deve ser maior que zero."
    ))]
    pub amount_cent: i32,
    pub due_date: NaiveDate,
    pub payment_date: Option<NaiveDate>,
    pub payment_method: PaymentMethod,
    #[validate(range(
        min = 1,
        max = 12,
        message = "O número da parcela deve estar entre 1 e 12."
    ))]
    pub installment_number: i32,
    pub status_transaction: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<FinancialRecords> for FinancialRecordsDto {
    fn from(entity: FinancialRecords) -> Self {
        Self {
            id: entity.id,
            contract_id: entity.contract_id,
            transaction_type: entity.transaction_type,
            description: entity.description,
            amount_cent: entity.amount_cent,
            due_date: entity.due_date,
            payment_date: entity.payment_date,
            payment_method: entity.payment_method,
            installment_number: entity.installment_number,
            status_transaction: entity.status_transaction,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl From<FinancialRecordsDto> for FinancialRecords {
    fn from(dto: FinancialRecordsDto) -> Self {
        Self {
            id: dto.id,
            contract_id: dto.contract_id,
            transaction_type: dto.transaction_type,
            description: dto.description,
            amount_cent: dto.amount_cent,
            due_date: dto.due_date,
            payment_date: dto.payment_date,
            payment_method: dto.payment_method,
            installment_number: dto.installment_number,
            status_transaction: dto.status_transaction,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialRecordsProjectionDto {
    pub id: Option<i64>,
    pub contract_id: Option<i64>,
    pub transaction_type: TransactionType,
    pub description: String,
    pub amount_cent: i32,
    pub due_date: NaiveDate,
    pub payment_date: Option<NaiveDate>,
    pub payment_method: PaymentMethod,
    pub installment_number: i32,
    pub status_transaction: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub student_id: Option<i64>,
    pub student_name: Option<String>,
}

impl From<FinancialRecordsProjection> for FinancialRecordsProjectionDto {
    fn from(projection: FinancialRecordsProjection) -> Self {
        Self {
            id: projection.id,
            contract_id: projection.contract_id,
            transaction_type: projection.transaction_type,
            description: projection.description,
            amount_cent: projection.amount_cent,
            due_date: projection.due_date,
            payment_date: projection.payment_date,
            payment_method: projection.payment_method,
            installment_number: projection.installment_number,
            status_transaction: projection.status_transaction,
            created_at: projection.created_at,
            updated_at: projection.updated_at,
            student_id: projection.student_id,
            student_name: projection.student_name,
        }
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FinancialMonthDto {
    pub month: String,
    pub income_cents: i64,
    pub expense_cents: i64,
    pub receivable_cents: i64,
    pub payable_cents: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FinancialContractOptionDto {
    pub value: i64,
    pub label: String,
}
