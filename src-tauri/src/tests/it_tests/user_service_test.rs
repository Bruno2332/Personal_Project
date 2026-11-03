use chrono::Utc;

use crate::{
    dtos::user_dto::UserDto, services::user_service, tests::utils::database::create_test_database,
};

fn user_dto() -> UserDto {
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

#[tokio::test]
async fn user_setup_insert_exists_find_and_update_use_real_sqlite() {
    let pool = create_test_database().await;
    assert!(!user_service::exists(&pool).await.unwrap());

    let created = user_service::insert(&pool, user_dto()).await.unwrap();
    let id = created.id.unwrap();
    assert!(user_service::exists(&pool).await.unwrap());
    assert_eq!(
        user_service::find_by_id(&pool, id).await.unwrap().username,
        "Personal de teste"
    );

    let mut changed = created;
    changed.username = "Personal atualizado".into();
    changed.cref = "CREF-98765".into();
    let updated = user_service::update(&pool, id, changed).await.unwrap();
    assert_eq!(updated.username, "Personal atualizado");
    assert_eq!(updated.cref, "CREF-98765");
}
