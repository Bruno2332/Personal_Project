use crate::models::financial_records::PaymentMethod;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum BillingType {
    Monthly = 0,
    Package = 1,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum ServiceType {
    Training = 0,
    PhysicalAssessment = 1,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum ContractStatus {
    Active = 0,
    Completed = 1,
    Cancelled = 2,
}
#[derive(Debug, FromRow, Deserialize)]
pub struct Contract {
    pub id: Option<i64>,
    pub student_id: i64,
    pub contract_description: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub duration_months: Option<i32>,
    pub service_type: ServiceType,
    pub billing_type: BillingType,
    pub amount_cents: i32,
    pub payment_method: PaymentMethod,
    pub installment_count: i32,
    pub first_due_date: Option<NaiveDate>,
    pub due_day: Option<i32>,
    pub sessions_per_week: Option<i32>,
    pub contract_status: ContractStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
