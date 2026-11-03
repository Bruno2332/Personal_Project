use crate::models::contract::{BillingType, Contract, ContractStatus, ServiceType};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::SqliteConnection;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ContractRepository: Send + Sync {
    async fn find_all_by_student_id(
        &mut self,
        student_id: i64,
    ) -> Result<Vec<Contract>, sqlx::Error>;

    async fn insert(&mut self, contract: Contract) -> Result<Contract, sqlx::Error>;

    async fn update(&mut self, contract: Contract, id: i64) -> Result<Contract, sqlx::Error>;

    async fn belongs_to_student(
        &mut self,
        contract_id: i64,
        student_id: i64,
    ) -> Result<bool, sqlx::Error>;

    async fn complete_expired(
        &mut self,
        today: NaiveDate,
        updated_at: DateTime<Utc>,
    ) -> Result<Vec<i64>, sqlx::Error>;
}

#[async_trait]
impl ContractRepository for SqliteConnection {
    async fn find_all_by_student_id(
        &mut self,
        student_id: i64,
    ) -> Result<Vec<Contract>, sqlx::Error> {
        sqlx::query_as::<_, Contract>(
            r#"
            SELECT *
            FROM contracts
            WHERE student_id = $1
            ORDER BY id DESC
            "#,
        )
        .bind(student_id)
        .fetch_all(&mut *self)
        .await
    }

    async fn insert(&mut self, contract: Contract) -> Result<Contract, sqlx::Error> {
        sqlx::query_as::<_, Contract>(
            r#"
            INSERT INTO contracts(
                student_id,
                contract_description,
                start_date,
                end_date,
                duration_months,
                service_type,
                billing_type,
                amount_cents,
                payment_method,
                installment_count,
                first_due_date,
                due_day,
                sessions_per_week,
                contract_status,
                created_at,
                updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *
            "#,
        )
        .bind(contract.student_id)
        .bind(contract.contract_description)
        .bind(contract.start_date)
        .bind(contract.end_date)
        .bind(contract.duration_months)
        .bind(contract.service_type)
        .bind(contract.billing_type)
        .bind(contract.amount_cents)
        .bind(contract.payment_method)
        .bind(contract.installment_count)
        .bind(contract.first_due_date)
        .bind(contract.due_day)
        .bind(contract.sessions_per_week)
        .bind(contract.contract_status)
        .bind(contract.created_at)
        .bind(contract.updated_at)
        .fetch_one(&mut *self)
        .await
    }

    async fn update(&mut self, contract: Contract, id: i64) -> Result<Contract, sqlx::Error> {
        sqlx::query_as::<_, Contract>(
            r#"
            UPDATE contracts
            SET
                student_id = $1,
                contract_description = $2,
                start_date = $3,
                end_date = $4,
                duration_months = $5,
                service_type = $6,
                billing_type = $7,
                amount_cents = $8,
                payment_method = $9,
                installment_count = $10,
                first_due_date = $11,
                due_day = $12,
                sessions_per_week = $13,
                contract_status = $14,
                updated_at = $15
            WHERE id = $16
            RETURNING *
            "#,
        )
        .bind(contract.student_id)
        .bind(contract.contract_description)
        .bind(contract.start_date)
        .bind(contract.end_date)
        .bind(contract.duration_months)
        .bind(contract.service_type)
        .bind(contract.billing_type)
        .bind(contract.amount_cents)
        .bind(contract.payment_method)
        .bind(contract.installment_count)
        .bind(contract.first_due_date)
        .bind(contract.due_day)
        .bind(contract.sessions_per_week)
        .bind(contract.contract_status)
        .bind(contract.updated_at)
        .bind(id)
        .fetch_one(&mut *self)
        .await
    }

    async fn belongs_to_student(
        &mut self,
        contract_id: i64,
        student_id: i64,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            r#"
        SELECT EXISTS(
            SELECT 1
            FROM contracts
            WHERE id = $1 AND student_id = $2
        )
        "#,
        )
        .bind(contract_id)
        .bind(student_id)
        .fetch_one(&mut *self)
        .await
    }

    async fn complete_expired(
        &mut self,
        today: NaiveDate,
        updated_at: DateTime<Utc>,
    ) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            r#"
        UPDATE contracts
        SET
            contract_status = $1,
            updated_at = $2
        WHERE contract_status = $3
          AND service_type = $4
          AND billing_type = $5
          AND end_date IS NOT NULL
          AND end_date < $6
        RETURNING student_id
        "#,
        )
            .bind(ContractStatus::Completed)
            .bind(updated_at)
            .bind(ContractStatus::Active)
            .bind(ServiceType::Training)
            .bind(BillingType::Package)
            .bind(today)
            .fetch_all(&mut *self)
            .await
    }
}
