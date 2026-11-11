use std::{io, path::Path};

use keyring::Entry;
use rand_core::{OsRng, RngCore};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions, Connection, SqliteConnection, SqlitePool,
};

pub async fn init(data_dir: &Path) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    std::fs::create_dir_all(data_dir)?;

    let filename = if cfg!(debug_assertions) {
        "personal-dev.db"
    } else {
        "personal.db"
    };

    let database_path = data_dir.join(filename);
    let database_exists = database_path.try_exists()?;

    // Confirma que SQLCipher está disponível antes de criar o banco.
    let probe_options = SqliteConnectOptions::new()
        .in_memory(true)
        .disable_statement_logging();

    let mut probe = SqliteConnection::connect_with(&probe_options).await?;

    let cipher_version = sqlx::query_scalar::<_, String>("PRAGMA cipher_version")
        .fetch_optional(&mut probe)
        .await?;

    probe.close().await?;

    if cipher_version
        .as_deref()
        .map_or(true, |version| version.trim().is_empty())
    {
        return Err(io::Error::other("O aplicativo não foi compilado com SQLCipher.").into());
    }

    // Cada banco possui sua própria entrada no cofre do sistema.
    let entry = Entry::new("personal_trainer_project.database", filename)?;

    let key = match entry.get_password() {
        Ok(key) => key,

        Err(keyring::Error::NoEntry) => {
            if database_exists {
                return Err(io::Error::other(
                    "O banco existe, mas sua chave não foi encontrada \
                     no cofre do sistema.",
                )
                .into());
            }

            let mut bytes = [0_u8; 32];

            OsRng
                .try_fill_bytes(&mut bytes)
                .map_err(|_| io::Error::other("Não foi possível gerar a chave do banco."))?;

            let key = hex::encode(bytes);

            entry.set_password(&key)?;

            key
        }

        Err(error) => return Err(error.into()),
    };

    if key.len() != 64 || !key.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(
            io::Error::other("A chave armazenada para o banco possui formato inválido.").into(),
        );
    }

    // A chave é aplicada antes de qualquer criação de tabela.
    let options = SqliteConnectOptions::new()
        .filename(&database_path)
        .create_if_missing(!database_exists)
        .pragma("key", format!("\"x'{}'\"", key))
        .foreign_keys(true)
        .with_regexp()
        .disable_statement_logging();

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;

    // Verifica se a chave consegue abrir o banco.
    if let Err(error) = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sqlite_master")
        .fetch_one(&pool)
        .await
    {
        pool.close().await;
        return Err(error.into());
    }

    // As tabelas já são criadas no banco criptografado.
    if let Err(error) = sqlx::migrate!("db/migrations").run(&pool).await {
        pool.close().await;
        return Err(error.into());
    }

    
    #[cfg(debug_assertions)]
    if std::env::var("RUN_DEV_SEED").as_deref() == Ok("1") {
        let seed: &'static str = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/db/seeds/development.sql"
        ));

        if let Err(error) = sqlx::raw_sql(seed).execute(&pool).await {
            pool.close().await;
            return Err(error.into());
        }

        println!("Seed de desenvolvimento executado.");
    }

    Ok(pool)
}
