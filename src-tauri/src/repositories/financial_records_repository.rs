use crate::dtos::financial_records_dto::{FinancialContractOptionDto, FinancialMonthDto};
use crate::models::contract::BillingType;
use crate::models::financial_records::{
    FinancialRecords, FinancialRecordsProjection, TransactionStatus, TransactionType,
};
use crate::utils::search::name_search_pattern;
use async_trait::async_trait;
use sqlx::{QueryBuilder, Sqlite, SqliteConnection, SqlitePool};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FinancialRecordsPoolRepository: Send + Sync {
    async fn find_all_paginated(
        &self,
        page: i64,
        size: i64,
        description: Option<String>,
        monthly: Option<bool>,
        transaction_type: Option<TransactionType>,
        status: Option<String>,
        period: Option<String>,
        contract_id: Option<i64>,
    ) -> Result<Vec<FinancialRecordsProjection>, sqlx::Error>;

    async fn count_all(
        &self,
        description: Option<String>,
        monthly: Option<bool>,
        transaction_type: Option<TransactionType>,
        status: Option<String>,
        period: Option<String>,
        contract_id: Option<i64>,
    ) -> Result<i64, sqlx::Error>;

    async fn summary_months(&self) -> Result<Vec<FinancialMonthDto>, sqlx::Error>;

    async fn contract_options(&self) -> Result<Vec<FinancialContractOptionDto>, sqlx::Error>;
}

#[async_trait]
impl FinancialRecordsPoolRepository for SqlitePool {
    async fn find_all_paginated(
        &self,
        page: i64,
        size: i64,
        description: Option<String>,
        monthly: Option<bool>,
        transaction_type: Option<TransactionType>,
        status: Option<String>,
        period: Option<String>,
        contract_id: Option<i64>,
    ) -> Result<Vec<FinancialRecordsProjection>, sqlx::Error> {
        let mut query = filtered_query(
            "SELECT f.*, c.student_id, s.name AS student_name",
            description,
            monthly,
            transaction_type,
            status,
            period,
            contract_id,
        );
        query
            .push(if monthly == Some(true) {
                " ORDER BY COALESCE(f.payment_date, f.due_date) DESC, f.id DESC LIMIT "
            } else {
                " ORDER BY COALESCE(f.payment_date, f.due_date) ASC, f.id ASC LIMIT "
            })
            .push_bind(size)
            .push(" OFFSET ")
            .push_bind(page * size);
        query
            .build_query_as::<FinancialRecordsProjection>()
            .fetch_all(self)
            .await
    }
    async fn count_all(
        &self,
        description: Option<String>,
        monthly: Option<bool>,
        transaction_type: Option<TransactionType>,
        status: Option<String>,
        period: Option<String>,
        contract_id: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
        filtered_query(
            "SELECT COUNT(*)",
            description,
            monthly,
            transaction_type,
            status,
            period,
            contract_id,
        )
        .build_query_scalar::<i64>()
        .fetch_one(self)
        .await
    }

    async fn summary_months(&self) -> Result<Vec<FinancialMonthDto>, sqlx::Error> {
        sqlx::query_as::<_, FinancialMonthDto>(r#"
        SELECT substr(COALESCE(payment_date, due_date), 1, 7) AS month,
            COALESCE(SUM(CASE WHEN transaction_type = $1 AND status_transaction = $3 THEN amount_cent ELSE 0 END), 0) AS income_cents,
            COALESCE(SUM(CASE WHEN transaction_type = $2 AND status_transaction = $3 THEN amount_cent ELSE 0 END), 0) AS expense_cents,
            COALESCE(SUM(CASE WHEN transaction_type = $1 AND status_transaction = $4 THEN amount_cent ELSE 0 END), 0) AS receivable_cents,
            COALESCE(SUM(CASE WHEN transaction_type = $2 AND status_transaction = $4 THEN amount_cent ELSE 0 END), 0) AS payable_cents
        FROM financial_records
        GROUP BY substr(COALESCE(payment_date, due_date), 1, 7)
        ORDER BY month
    "#
        )
            .bind(TransactionType::Income)
            .bind(TransactionType::Expense)
            .bind(TransactionStatus::Paid)
            .bind(TransactionStatus::Pending)
            .fetch_all(self)
            .await
    }

    async fn contract_options(&self) -> Result<Vec<FinancialContractOptionDto>, sqlx::Error> {
        sqlx::query_as::<_, FinancialContractOptionDto>(
            r#"
        SELECT c.id AS value, COALESCE(s.name, 'Aluno') || ' — ' || c.contract_description AS label
        FROM contracts c LEFT JOIN students s ON s.id = c.student_id
        ORDER BY s.name, c.start_date DESC, c.id DESC
    "#,
        )
        .fetch_all(self)
        .await
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FinancialRecordsConnectionRepository: Send + Sync {
    async fn insert(&mut self, record: FinancialRecords) -> Result<FinancialRecords, sqlx::Error>;

    async fn update(
        &mut self,
        id: i64,
        record: FinancialRecords,
    ) -> Result<FinancialRecords, sqlx::Error>;

    async fn find_all_by_contract_id(
        &mut self,
        contract_id: i64,
    ) -> Result<Vec<FinancialRecords>, sqlx::Error>;

    async fn delete_pending_by_id(&mut self, id: i64) -> Result<(), sqlx::Error>;

    async fn generate_monthly_charges(
        &mut self,
        today: chrono::NaiveDate,
    ) -> Result<u64, sqlx::Error>;
}

#[async_trait]
impl FinancialRecordsConnectionRepository for SqliteConnection {
    async fn insert(&mut self, record: FinancialRecords) -> Result<FinancialRecords, sqlx::Error> {
        sqlx::query_as::<_, FinancialRecords>(
            r#"
        INSERT INTO financial_records (
            contract_id,
            transaction_type,
            description,
            amount_cent,
            due_date,
            payment_date,
            payment_method,
            installment_number,
            status_transaction,
            created_at,
            updated_at,
            competence_month
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
            CASE
                WHEN $2 = 0 AND EXISTS (
                    SELECT 1
                    FROM contracts c
                    WHERE c.id = $1
                      AND c.service_type = 0
                      AND c.billing_type = 0
                )
                THEN substr($5, 1, 7)
                ELSE NULL
            END
        )
        RETURNING *
        "#,
        )
        .bind(record.contract_id)
        .bind(record.transaction_type)
        .bind(record.description)
        .bind(record.amount_cent)
        .bind(record.due_date)
        .bind(record.payment_date)
        .bind(record.payment_method)
        .bind(record.installment_number)
        .bind(record.status_transaction)
        .bind(record.created_at)
        .bind(record.updated_at)
        .fetch_one(&mut *self)
        .await
    }

    async fn update(
        &mut self,
        id: i64,
        record: FinancialRecords,
    ) -> Result<FinancialRecords, sqlx::Error> {
        sqlx::query_as::<_, FinancialRecords>(
            r#"
            UPDATE financial_records
            SET
                contract_id = $1,
                transaction_type = $2,
                description = $3,
                amount_cent = $4,
                due_date = $5,
                payment_date = $6,
                payment_method = $7,
                installment_number = $8,
                status_transaction = $9,
                updated_at = $10
                WHERE id = $11
                RETURNING *
            "#,
        )
        .bind(record.contract_id)
        .bind(record.transaction_type)
        .bind(record.description)
        .bind(record.amount_cent)
        .bind(record.due_date)
        .bind(record.payment_date)
        .bind(record.payment_method)
        .bind(record.installment_number)
        .bind(record.status_transaction)
        .bind(record.updated_at)
        .bind(id)
        .fetch_one(&mut *self)
        .await
    }

    async fn find_all_by_contract_id(
        &mut self,
        contract_id: i64,
    ) -> Result<Vec<FinancialRecords>, sqlx::Error> {
        sqlx::query_as::<_, FinancialRecords>(
            r#"
        SELECT *
        FROM financial_records
        WHERE contract_id = $1
        ORDER BY installment_number, id
        "#,
        )
        .bind(contract_id)
        .fetch_all(&mut *self)
        .await
    }

    async fn delete_pending_by_id(&mut self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        DELETE FROM financial_records
        WHERE id = $1
          AND status_transaction = $2
        "#,
        )
        .bind(id)
        .bind(TransactionStatus::Pending)
        .execute(&mut *self)
        .await?;

        Ok(())
    }

    async fn generate_monthly_charges(
        &mut self,
        today: chrono::NaiveDate,
    ) -> Result<u64, sqlx::Error> {
        let competence = today.format("%Y-%m").to_string();

        let result = sqlx::query(
            r#"
        WITH monthly_contracts AS (
            SELECT
                c.id AS contract_id,
                c.contract_description,
                c.amount_cents,
                c.payment_method,
                s.name AS student_name,
                c.start_date,
                c.end_date,
                CASE
                    WHEN substr(c.first_due_date, 1, 7) = $2
                        THEN c.first_due_date
                    ELSE printf('%s-%02d', $2, c.due_day)
                END AS due_date
            FROM contracts c
            INNER JOIN students s ON s.id = c.student_id
            WHERE s.is_active = 1
              AND c.contract_status = 0
              AND c.service_type = 0
              AND c.billing_type = 0
              AND c.due_day BETWEEN 1 AND 28
              AND c.start_date <= $1
              AND (
                  c.end_date IS NULL
                  OR c.end_date >= $2 || '-01'
              )
              AND (
                  c.first_due_date IS NULL
                  OR substr(c.first_due_date, 1, 7) <= $2
              )
        )
        INSERT INTO financial_records (
            contract_id,
            transaction_type,
            description,
            amount_cent,
            due_date,
            payment_date,
            payment_method,
            installment_number,
            status_transaction,
            competence_month,
            created_at,
            updated_at
        )
        SELECT
            c.contract_id,
            0,
            'Mensalidade ' || $2 || ' - ' ||
                c.student_name || ' - ' || c.contract_description,
            c.amount_cents,
            c.due_date,
            NULL,
            c.payment_method,
            1,
            0,
            $2,
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        FROM monthly_contracts c
        WHERE c.due_date >= c.start_date
          AND (
              c.end_date IS NULL
              OR c.due_date <= c.end_date
          )
          AND NOT EXISTS (
              SELECT 1
              FROM financial_records f
              WHERE f.contract_id = c.contract_id
                AND f.competence_month = $2
          )
        ON CONFLICT (contract_id, competence_month) DO NOTHING
        "#,
        )
        .bind(today)
        .bind(competence)
        .execute(&mut *self)
        .await?;

        Ok(result.rows_affected())
    }
}

fn filtered_query(
    select: &str,
    description: Option<String>,
    monthly: Option<bool>,
    transaction_type: Option<TransactionType>,
    status: Option<String>,
    period: Option<String>,
    contract_id: Option<i64>,
) -> QueryBuilder<Sqlite> {
    let mut query = QueryBuilder::new(format!(
        "{select} FROM financial_records f LEFT JOIN contracts c ON c.id = f.contract_id LEFT JOIN students s ON s.id = c.student_id WHERE 1=1"
    ));

    if let Some(text) = description
        .as_deref()
        .filter(|text| !text.trim().is_empty())
    {
        let pattern = name_search_pattern(text);
        query
            .push(" AND (f.description REGEXP ")
            .push_bind(pattern.clone())
            .push(" OR s.name REGEXP ")
            .push_bind(pattern)
            .push(")");
    }

    if let Some(monthly) = monthly {
        query
            .push(" AND EXISTS (SELECT 1 FROM contracts mc WHERE mc.id = f.contract_id AND mc.billing_type = ")
            .push_bind(BillingType::Monthly)
            .push(") = ")
            .push_bind(monthly);
    }

    if let Some(kind) = transaction_type {
        query.push(" AND f.transaction_type = ").push_bind(kind);
    }

    if let Some(status) = status {
        match status.as_str() {
            "Paid" => {
                query
                    .push(" AND f.status_transaction = ")
                    .push_bind(TransactionStatus::Paid);
            }
            "Cancelled" => {
                query
                    .push(" AND f.status_transaction = ")
                    .push_bind(TransactionStatus::Cancelled);
            }
            "Pending" | "Overdue" => {
                query
                    .push(" AND f.status_transaction = ")
                    .push_bind(TransactionStatus::Pending);
                query
                    .push(if status == "Overdue" {
                        " AND f.due_date < "
                    } else {
                        " AND f.due_date >= "
                    })
                    .push_bind(chrono::Local::now().date_naive());
            }
            _ => {
                query.push(" AND 0=1");
            }
        }
    }

    if let Some(period) = period {
        let months_back = match period.as_str() {
            "1m" => Some(1),
            "3m" => Some(3),
            "6m" => Some(6),
            "12m" => Some(12),
            _ => None,
        };

        if let Some(months_back) = months_back {
            let today = chrono::Local::now().date_naive();
            let cutoff_month = today
                .checked_sub_months(chrono::Months::new(months_back))
                .unwrap_or(today)
                .format("%Y-%m")
                .to_string();

            query
                .push(
                    " AND COALESCE(
                        f.competence_month,
                        substr(f.due_date, 1, 7)
                    ) >= ",
                )
                .push_bind(cutoff_month);
        } else {
            query.push(" AND 0=1");
        }
    }

    if let Some(id) = contract_id {
        query.push(" AND f.contract_id = ").push_bind(id);
    }

    query
}
