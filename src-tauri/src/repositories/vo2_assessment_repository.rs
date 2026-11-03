use crate::models::vo2_assessment::Vo2Assessment;
use async_trait::async_trait;
use sqlx::{SqliteConnection, SqlitePool};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Vo2AssessmentPoolRepository: Send + Sync {
    async fn find_all_by_student_id(
        &self,
        student_id: i64,
    ) -> Result<Vec<Vo2Assessment>, sqlx::Error>;
}

#[async_trait]
impl Vo2AssessmentPoolRepository for SqlitePool {
    async fn find_all_by_student_id(
        &self,
        student_id: i64,
    ) -> Result<Vec<Vo2Assessment>, sqlx::Error> {
        sqlx::query_as::<_, Vo2Assessment>(
            r#"
        SELECT v.*
        FROM vo2_assessments v
        INNER JOIN physical_assessments p
            ON p.id = v.physical_assessment_id
        WHERE p.student_id = $1
        ORDER BY p.assessment_date, p.id, v.treadmill_speed_kmh, v.id
        "#,
        )
        .bind(student_id)
        .fetch_all(self)
        .await
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Vo2AssessmentConnectionRepository: Send + Sync {
    async fn insert_many(&mut self, entities: Vec<Vo2Assessment>) -> Result<u64, sqlx::Error>;

    async fn update_many(
        &mut self,
        physical_assessment_id: i64,
        entities: Vec<Vo2Assessment>,
    ) -> Result<u64, sqlx::Error>;

    async fn delete_by_ids(
        &mut self,
        ids: Vec<i64>,
        physical_assessment_id: i64,
    ) -> Result<u64, sqlx::Error>;
}

#[async_trait]
impl Vo2AssessmentConnectionRepository for SqliteConnection {
    async fn insert_many(&mut self, entities: Vec<Vo2Assessment>) -> Result<u64, sqlx::Error> {
        if entities.is_empty() {
            return Ok(0);
        }

        let mut query = sqlx::QueryBuilder::<sqlx::Sqlite>::new(
            r#"
        INSERT INTO vo2_assessments (
            physical_assessment_id,
            treadmill_speed_kmh,
            heart_rate_bpm,
            perceived_exertion,
            created_at,
            updated_at
        )
        "#,
        );

        query.push_values(entities, |mut row, entity| {
            row.push_bind(entity.physical_assessment_id)
                .push_bind(entity.treadmill_speed_kmh)
                .push_bind(entity.heart_rate_bpm)
                .push_bind(entity.perceived_exertion)
                .push_bind(entity.created_at)
                .push_bind(entity.updated_at);
        });

        let result = query.build().execute(&mut *self).await?;

        Ok(result.rows_affected())
    }

    async fn update_many(
        &mut self,
        physical_assessment_id: i64,
        entities: Vec<Vo2Assessment>,
    ) -> Result<u64, sqlx::Error> {
        if entities.is_empty() {
            return Ok(0);
        }

        let mut ids = std::collections::HashSet::new();

        for entity in &entities {
            let id = entity.id.ok_or_else(|| {
                sqlx::Error::Protocol("Etapa sem ID na atualização em lote.".to_string())
            })?;

            if !ids.insert(id) {
                return Err(sqlx::Error::Protocol(
                    "ID de etapa repetido na atualização em lote.".to_string(),
                ));
            }
        }

        let expected = entities.len() as u64;

        let mut query = sqlx::QueryBuilder::<sqlx::Sqlite>::new(
            "UPDATE vo2_assessments SET treadmill_speed_kmh = CASE id",
        );

        for entity in &entities {
            query
                .push(" WHEN ")
                .push_bind(entity.id)
                .push(" THEN ")
                .push_bind(entity.treadmill_speed_kmh);
        }

        query.push(" END, heart_rate_bpm = CASE id");

        for entity in &entities {
            query
                .push(" WHEN ")
                .push_bind(entity.id)
                .push(" THEN ")
                .push_bind(entity.heart_rate_bpm);
        }

        query.push(" END, perceived_exertion = CASE id");

        for entity in &entities {
            query
                .push(" WHEN ")
                .push_bind(entity.id)
                .push(" THEN ")
                .push_bind(&entity.perceived_exertion);
        }

        query.push(" END, updated_at = CASE id");

        for entity in &entities {
            query
                .push(" WHEN ")
                .push_bind(entity.id)
                .push(" THEN ")
                .push_bind(entity.updated_at);
        }

        query
            .push(" END WHERE physical_assessment_id = ")
            .push_bind(physical_assessment_id)
            .push(" AND id IN (");

        {
            let mut separated = query.separated(", ");

            for id in ids {
                separated.push_bind(id);
            }
        }

        query.push(")");

        let result = query.build().execute(&mut *self).await?;

        if result.rows_affected() != expected {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(result.rows_affected())
    }

    async fn delete_by_ids(
        &mut self,
        ids: Vec<i64>,
        physical_assessment_id: i64,
    ) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }

        let mut query = sqlx::QueryBuilder::<sqlx::Sqlite>::new(
            "DELETE FROM vo2_assessments WHERE physical_assessment_id = ",
        );

        query.push_bind(physical_assessment_id).push(" AND id IN (");

        {
            let mut separated = query.separated(", ");

            for id in ids {
                separated.push_bind(id);
            }
        }

        query.push(")");

        let result = query.build().execute(&mut *self).await?;

        Ok(result.rows_affected())
    }
}
