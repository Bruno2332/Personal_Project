use crate::models::agenda::Agenda;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::SqlitePool;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AgendaRepository: Send + Sync {
    async fn find_by_period(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Agenda>, sqlx::Error>;

    async fn insert(&self, entity: Agenda) -> Result<Agenda, sqlx::Error>;

    async fn update(&self, entity: Agenda, id: i64) -> Result<Agenda, sqlx::Error>;

    async fn delete_by_id(&self, id: i64) -> Result<u64, sqlx::Error>;
}

#[async_trait]
impl AgendaRepository for SqlitePool {
    async fn find_by_period(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<Agenda>, sqlx::Error> {
        sqlx::query_as::<_, Agenda>(
            r#"
        SELECT *
        FROM agenda
        WHERE
            (
                is_recurring = 0
                AND scheduled_date BETWEEN $1 AND $2
            )
            OR
            (
                is_recurring = 1
                AND recurrence_start_date <= $2
                AND (
                    recurrence_end_date IS NULL
                    OR recurrence_end_date >= $1
                )
            )
        ORDER BY start_time, id
        "#,
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(self)
        .await
    }

    async fn insert(&self, entity: Agenda) -> Result<Agenda, sqlx::Error> {
        sqlx::query_as::<_, Agenda>(
            r#"
        INSERT INTO agenda (
            student_id,
            appointment_type,
            is_recurring,
            scheduled_date,
            week_day,
            start_time,
            end_time,
            recurrence_start_date,
            recurrence_end_date,
            notes,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
        )
        .bind(entity.student_id)
        .bind(entity.appointment_type)
        .bind(entity.is_recurring)
        .bind(entity.scheduled_date)
        .bind(entity.week_day)
        .bind(entity.start_time)
        .bind(entity.end_time)
        .bind(entity.recurrence_start_date)
        .bind(entity.recurrence_end_date)
        .bind(entity.notes)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .fetch_one(self)
        .await
    }

    async fn update(&self, entity: Agenda, id: i64) -> Result<Agenda, sqlx::Error> {
        sqlx::query_as::<_, Agenda>(
            r#"
        UPDATE agenda
        SET
            student_id = $1,
            appointment_type = $2,
            is_recurring = $3,
            scheduled_date = $4,
            week_day = $5,
            start_time = $6,
            end_time = $7,
            recurrence_start_date = $8,
            recurrence_end_date = $9,
            notes = $10,
            updated_at = $11
        WHERE id = $12
        RETURNING *
        "#,
        )
        .bind(entity.student_id)
        .bind(entity.appointment_type)
        .bind(entity.is_recurring)
        .bind(entity.scheduled_date)
        .bind(entity.week_day)
        .bind(entity.start_time)
        .bind(entity.end_time)
        .bind(entity.recurrence_start_date)
        .bind(entity.recurrence_end_date)
        .bind(entity.notes)
        .bind(entity.updated_at)
        .bind(id)
        .fetch_one(self)
        .await
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
        DELETE FROM agenda
        WHERE id = $1
        "#,
        )
        .bind(id)
        .execute(self)
        .await?;

        Ok(result.rows_affected())
    }
}
