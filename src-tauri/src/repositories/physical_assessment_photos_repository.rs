use crate::models::physical_assessment_photos::{PhotoPosition, PhysicalAssessmentPhotos};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PhysicalAssessmentPhotosRepository: Send + Sync {
    async fn find_all_by_assessment_id(
        &self,
        assessment_id: i64,
    ) -> Result<Vec<PhysicalAssessmentPhotos>, sqlx::Error>;

    async fn find_by_id(&self, id: i64) -> Result<Option<PhysicalAssessmentPhotos>, sqlx::Error>;

    async fn insert(
        &self,
        entity: PhysicalAssessmentPhotos,
    ) -> Result<PhysicalAssessmentPhotos, sqlx::Error>;

    async fn update(
        &self,
        id: i64,
        position: PhotoPosition,
        description: Option<String>,
        updated_at: DateTime<Utc>,
    ) -> Result<PhysicalAssessmentPhotos, sqlx::Error>;

    async fn delete_by_id(&self, id: i64) -> Result<u64, sqlx::Error>;
}

#[async_trait]
impl PhysicalAssessmentPhotosRepository for SqlitePool {
    async fn find_all_by_assessment_id(
        &self,
        assessment_id: i64,
    ) -> Result<Vec<PhysicalAssessmentPhotos>, sqlx::Error> {
        sqlx::query_as::<_, PhysicalAssessmentPhotos>(
            r#"
            SELECT *
            FROM physical_assessment_photos
            WHERE physical_assessment_id = $1
            ORDER BY created_at, id
            "#,
        )
        .bind(assessment_id)
        .fetch_all(self)
        .await
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<PhysicalAssessmentPhotos>, sqlx::Error> {
        sqlx::query_as::<_, PhysicalAssessmentPhotos>(
            r#"
            SELECT *
            FROM physical_assessment_photos
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self)
        .await
    }

    async fn insert(
        &self,
        entity: PhysicalAssessmentPhotos,
    ) -> Result<PhysicalAssessmentPhotos, sqlx::Error> {
        sqlx::query_as::<_, PhysicalAssessmentPhotos>(
            r#"
            INSERT INTO physical_assessment_photos (
                physical_assessment_id,
                file_path,
                position,
                description,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(entity.physical_assessment_id)
        .bind(entity.file_path)
        .bind(entity.position)
        .bind(entity.description)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .fetch_one(self)
        .await
    }

    async fn update(
        &self,
        id: i64,
        position: PhotoPosition,
        description: Option<String>,
        updated_at: DateTime<Utc>,
    ) -> Result<PhysicalAssessmentPhotos, sqlx::Error> {
        sqlx::query_as::<_, PhysicalAssessmentPhotos>(
            r#"
            UPDATE physical_assessment_photos
            SET
                position = $1,
                description = $2,
                updated_at = $3
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(position)
        .bind(description)
        .bind(updated_at)
        .bind(id)
        .fetch_one(self)
        .await
    }

    async fn delete_by_id(&self, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM physical_assessment_photos
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(self)
        .await?;

        Ok(result.rows_affected())
    }
}
