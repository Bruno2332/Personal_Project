use crate::dtos::financial_records_dto::FinancialRecordsDto;
use crate::models::contract::{BillingType, Contract, ContractStatus, ServiceType};
use crate::models::financial_records::PaymentMethod;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ContractDto {
    pub id: Option<i64>,
    pub student_id: i64,
    pub contract_description: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    #[validate(range(
        min = 1,
        max = 99,
        message = "Informe um número positivo de até 2 dígitos."
    ))]
    pub duration_months: Option<i32>,
    pub service_type: ServiceType,
    pub billing_type: BillingType,
    #[validate(range(
        min = 1,
        message = "O valor do contrato deve ser maior que zero."
    ))]
    pub amount_cents: i32,
    pub payment_method: PaymentMethod,
    #[validate(range(
        min = 1,
        max = 12,
        message = "A quantidade de parcelas deve ser de 1 a 12."
    ))]
    pub installment_count: i32,
    pub first_due_date: Option<NaiveDate>,
    #[validate(range(
        min = 1,
        max = 28,
        message = "O dia do vencimento deve ser de 1 a 28."
    ))]
    pub due_day: Option<i32>,
    #[validate(range(
        min = 1,
        max = 7,
        message = "A quantidade de aulas por semana deve ser de 1 a 7."
    ))]
    pub sessions_per_week: Option<i32>,
    pub contract_status: ContractStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Contract> for ContractDto {
    fn from(contract: Contract) -> Self {
        Self {
            id: contract.id,
            student_id: contract.student_id,
            contract_description: contract.contract_description,
            start_date: contract.start_date,
            end_date: contract.end_date,
            duration_months: contract.duration_months,
            service_type: contract.service_type,
            billing_type: contract.billing_type,
            amount_cents: contract.amount_cents,
            payment_method: contract.payment_method,
            installment_count: contract.installment_count,
            first_due_date: contract.first_due_date,
            due_day: contract.due_day,
            sessions_per_week: contract.sessions_per_week,
            contract_status: contract.contract_status,
            created_at: contract.created_at,
            updated_at: contract.updated_at,
        }
    }
}

impl From<ContractDto> for Contract {
    fn from(dto: ContractDto) -> Self {
        Self {
            id: dto.id,
            student_id: dto.student_id,
            contract_description: dto.contract_description,
            start_date: dto.start_date,
            end_date: dto.end_date,
            duration_months: dto.duration_months,
            service_type: dto.service_type,
            billing_type: dto.billing_type,
            amount_cents: dto.amount_cents,
            payment_method: dto.payment_method,
            installment_count: dto.installment_count,
            first_due_date: dto.first_due_date,
            due_day: dto.due_day,
            sessions_per_week: dto.sessions_per_week,
            contract_status: dto.contract_status,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateContractRequest {
    pub contract_dto: ContractDto,
    pub financial_record: FinancialRecordsDto,
}
