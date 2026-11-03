use chrono::{NaiveDate, Utc};

use crate::{
    dtos::financial_records_dto::{
        FinancialContractOptionDto, FinancialMonthDto, FinancialRecordsDto,
    },
    models::financial_records::{
        FinancialRecords, FinancialRecordsProjection, PaymentMethod, TransactionStatus,
        TransactionType,
    },
};

pub fn valid_financial_record_dto() -> FinancialRecordsDto {
    let now = Utc::now();

    FinancialRecordsDto {
        id: None,
        contract_id: None,
        transaction_type: TransactionType::Income,
        description: "Mensalidade de teste".to_string(),
        amount_cent: 12_500,
        due_date: NaiveDate::from_ymd_opt(2026, 11, 10).unwrap(),
        payment_date: None,
        payment_method: PaymentMethod::Pix,
        installment_number: 1,
        status_transaction: TransactionStatus::Pending,
        created_at: now,
        updated_at: now,
    }
}

pub fn financial_record(
    id: Option<i64>,
    installment_number: i32,
    status_transaction: TransactionStatus,
) -> FinancialRecords {
    let now = Utc::now();

    FinancialRecords {
        id,
        contract_id: Some(20),
        transaction_type: TransactionType::Income,
        description: format!("Parcela {installment_number}"),
        amount_cent: 5_000,
        due_date: NaiveDate::from_ymd_opt(2026, 11, 10).unwrap(),
        payment_date: None,
        payment_method: PaymentMethod::Pix,
        installment_number,
        status_transaction,
        created_at: now,
        updated_at: now,
    }
}

pub fn financial_record_projection() -> FinancialRecordsProjection {
    let record = financial_record(Some(5), 2, TransactionStatus::Paid);

    FinancialRecordsProjection {
        id: record.id,
        contract_id: record.contract_id,
        transaction_type: record.transaction_type,
        description: record.description,
        amount_cent: record.amount_cent,
        due_date: record.due_date,
        payment_date: Some(NaiveDate::from_ymd_opt(2026, 11, 9).unwrap()),
        payment_method: record.payment_method,
        installment_number: record.installment_number,
        status_transaction: record.status_transaction,
        created_at: record.created_at,
        updated_at: record.updated_at,
        student_id: Some(7),
        student_name: Some("Aluno de teste".to_string()),
    }
}

pub fn financial_month() -> FinancialMonthDto {
    FinancialMonthDto {
        month: "2026-11".to_string(),
        income_cents: 20_000,
        expense_cents: 5_000,
        receivable_cents: 10_000,
        payable_cents: 2_000,
    }
}

pub fn financial_contract_option() -> FinancialContractOptionDto {
    FinancialContractOptionDto {
        value: 20,
        label: "Aluno de teste — Plano mensal".to_string(),
    }
}
