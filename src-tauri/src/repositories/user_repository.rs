use crate::models::user::User;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error>;

    async fn exists(&self) -> Result<bool, sqlx::Error>;

    async fn insert(&self, entity: User) -> Result<User, sqlx::Error>;

    async fn update(&self, entity: User, id: i64) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserRepository for SqlitePool {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
		SELECT *
		FROM users
		WHERE id = $1
		"#,
        )
        .bind(id)
        .fetch_optional(self)
        .await
    }

    async fn exists(&self) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users)")
            .fetch_one(self)
            .await
    }

    async fn insert(&self, entity: User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                username,
                email,
                cref,
                created_at,
                updated_at
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(entity.username)
        .bind(entity.email)
        .bind(entity.cref)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .fetch_one(self)
        .await
    }

    async fn update(&self, entity: User, id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET
                username = $1,
                email = $2,
                cref = $3,
                updated_at = $4
            WHERE id = $5
            RETURNING *
            "#,
        )
        .bind(entity.username)
        .bind(entity.email)
        .bind(entity.cref)
        .bind(entity.updated_at)
        .bind(id)
        .fetch_one(self)
        .await
    }
}
