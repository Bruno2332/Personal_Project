use chrono::{NaiveDate, Utc};

use crate::{
    dtos::{
        contract_dto::{ContractDto, CreateContractRequest},
        financial_records_dto::FinancialRecordsDto,
    },
    models::{
        contract::{BillingType, Contract, ContractStatus, ServiceType},
        financial_records::{PaymentMethod, TransactionStatus, TransactionType},
    },
};

pub fn valid_contract_dto() -> ContractDto {
    let now = Utc::now();

    ContractDto {
        id: None,
        student_id: 7,
        contract_description: "Pacote de treinamento".to_string(),
        start_date: NaiveDate::from_ymd_opt(2026, 11, 1).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2027, 1, 31).unwrap()),
        duration_months: Some(6),
        service_type: ServiceType::Training,
        billing_type: BillingType::Package,
        amount_cents: 10_000,
        payment_method: PaymentMethod::Pix,
        installment_count: 3,
        first_due_date: Some(NaiveDate::from_ymd_opt(2026, 11, 10).unwrap()),
        due_day: Some(10),
        sessions_per_week: Some(3),
        contract_status: ContractStatus::Active,
        created_at: now,
        updated_at: now,
    }
}

pub fn valid_financial_record_dto() -> FinancialRecordsDto {
    let now = Utc::now();

    FinancialRecordsDto {
        id: None,
        contract_id: None,
        transaction_type: TransactionType::Income,
        description: "Pagamento do pacote".to_string(),
        amount_cent: 10_000,
        due_date: NaiveDate::from_ymd_opt(2026, 11, 10).unwrap(),
        payment_date: None,
        payment_method: PaymentMethod::Pix,
        installment_number: 1,
        status_transaction: TransactionStatus::Pending,
        created_at: now,
        updated_at: now,
    }
}

pub fn valid_create_contract_request() -> CreateContractRequest {
    CreateContractRequest {
        contract_dto: valid_contract_dto(),
        financial_record: valid_financial_record_dto(),
    }
}

pub fn valid_contract() -> Contract {
    Contract::from(valid_contract_dto())
}
