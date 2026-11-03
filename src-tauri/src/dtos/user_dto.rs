use crate::models::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub cref: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            cref: user.cref,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<UserDto> for User {
    fn from(dto: UserDto) -> Self {
        Self {
            id: dto.id,
            username: dto.username,
            email: dto.email,
            cref: dto.cref,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}
