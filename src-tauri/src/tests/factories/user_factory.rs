use chrono::Utc;

use crate::{dtos::user_dto::UserDto, models::user::User};

pub fn valid_user() -> User {
    let now = Utc::now();

    User {
        id: Some(7),
        username: "Personal de teste".into(),
        email: "personal@example.com".into(),
        cref: "CREF-12345".into(),
        created_at: now,
        updated_at: now,
    }
}

pub fn valid_user_dto() -> UserDto {
    let now = Utc::now();

    UserDto {
        id: None,
        username: "Personal de teste".into(),
        email: "personal@example.com".into(),
        cref: "CREF-12345".into(),
        created_at: now,
        updated_at: now,
    }
}
