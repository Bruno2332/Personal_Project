use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::SqlitePool;
use crate::models::agenda_exclusions::AgendaExclusion;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AgendaExclusionRepository: Send + Sync {
    async fn find_by_period(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<AgendaExclusion>, sqlx::Error>;

    async fn insert(
        &self,
        entity: AgendaExclusion,
    ) -> Result<AgendaExclusion, sqlx::Error>;
}

#[async_trait]
impl AgendaExclusionRepository for SqlitePool {
    async fn find_by_period(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<AgendaExclusion>, sqlx::Error> {
        sqlx::query_as::<_, AgendaExclusion>(
            r#"
            SELECT
                id,
                agenda_id,
                occurrence_date,
                created_at
            FROM agenda_exclusions
            WHERE occurrence_date BETWEEN $1 AND $2
            ORDER BY occurrence_date, agenda_id
            "#,
        )
            .bind(start_date)
            .bind(end_date)
            .fetch_all(self)
            .await
    }

    async fn insert(
        &self,
        entity: AgendaExclusion,
    ) -> Result<AgendaExclusion, sqlx::Error> {
        sqlx::query_as::<_, AgendaExclusion>(
            r#"
            INSERT INTO agenda_exclusions (
                agenda_id,
                occurrence_date,
                created_at
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (agenda_id, occurrence_date)
            DO UPDATE SET occurrence_date = excluded.occurrence_date
            RETURNING *
            "#,
        )
            .bind(entity.agenda_id)
            .bind(entity.occurrence_date)
            .bind(entity.created_at)
            .fetch_one(self)
            .await
    }
}