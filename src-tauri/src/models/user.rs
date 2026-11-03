use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub cref: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
