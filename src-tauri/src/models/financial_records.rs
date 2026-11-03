use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum TransactionType {
    Income = 0,
    Expense = 1,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum PaymentMethod {
    Pix = 0,
    CreditCard = 1,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum TransactionStatus {
    Pending = 0,
    Paid = 1,
    Cancelled = 2,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FinancialRecords {
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
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FinancialRecordsProjection {
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
