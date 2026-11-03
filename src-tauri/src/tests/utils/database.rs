use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

pub async fn create_test_database() -> SqlitePool {
    let options = SqliteConnectOptions::new()
        .in_memory(true)
        .foreign_keys(true)
        .with_regexp();

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .min_connections(1)
        .max_lifetime(None)
        .idle_timeout(None)
        .connect_with(options)
        .await
        .expect("Não foi possível criar o banco de teste");

    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("Não foi possível executar as migrations no banco de teste");

    pool
}
