use async_trait::async_trait;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    ConnectOptions, Connection, SqliteConnection, SqlitePool,
};
use std::path::Path;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait BackupRepository: Send + Sync {
    async fn create_snapshot(&self, destination: &Path) -> Result<(), String>;

    async fn validate_database(&self, path: &Path, key: &str) -> Result<(), String>;

    async fn count_required_tables(&self, path: &Path, key: &str) -> Result<i64, String>;

    async fn run_migrations(&self, path: &Path, key: &str) -> Result<(), String>;

    async fn change_database_key(
        &self,
        path: &Path,
        imported_key: &str,
        local_key: &str,
    ) -> Result<(), String>;

    async fn find_photo_paths(
        &self,
        database_path: &Path,
        key: &str,
    ) -> Result<Vec<String>, String>;

    async fn update_photo_path(
        &self,
        database_path: &Path,
        key: &str,
        previous_path: String,
        new_path: String,
    ) -> Result<(), String>;

    async fn close_for_restore(&self, database_path: &Path, key: &str) -> Result<(), String>;
}

#[async_trait]
impl BackupRepository for SqlitePool {
    async fn create_snapshot(&self, destination: &Path) -> Result<(), String> {
        let filename = destination
            .to_str()
            .ok_or_else(|| "O caminho da cópia do banco é inválido.".to_string())?;

        sqlx::query("VACUUM main INTO ?")
            .bind(filename)
            .execute(self)
            .await
            .map_err(|_| "Não foi possível criar a cópia do banco.".to_string())?;

        Ok(())
    }

    async fn validate_database(&self, path: &Path, key: &str) -> Result<(), String> {
        let mut connection = SqliteConnection::connect_with(&connection_options(path, key))
            .await
            .map_err(|_| "Não foi possível abrir o banco com a chave informada.".to_string())?;

        let verification = check_integrity(&mut connection).await;
        let closed = connection.close().await;

        verification?;

        closed.map_err(|_| "Não foi possível fechar a conexão de verificação.".to_string())?;

        Ok(())
    }

    async fn count_required_tables(&self, path: &Path, key: &str) -> Result<i64, String> {
        let mut connection = SqliteConnection::connect_with(&connection_options(path, key))
            .await
            .map_err(|_| "Não foi possível abrir o banco para consultar as tabelas.".to_string())?;

        let result = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM sqlite_master
            WHERE type = 'table'
              AND name IN (
                  'students',
                  'contracts',
                  'financial_records',
                  'agenda',
                  'physical_assessments',
                  'vo2_assessments',
                  'users',
                  '_sqlx_migrations'
              )
            "#,
        )
        .fetch_one(&mut connection)
        .await
        .map_err(|_| "Não foi possível verificar as tabelas do backup.".to_string());

        let closed = connection.close().await;

        let count = result?;

        closed.map_err(|_| "Não foi possível fechar a conexão de consulta.".to_string())?;

        Ok(count)
    }

    async fn run_migrations(&self, path: &Path, key: &str) -> Result<(), String> {
        let mut connection = SqliteConnection::connect_with(&connection_options(path, key))
            .await
            .map_err(|_| {
                "Não foi possível abrir o banco para executar as migrations.".to_string()
            })?;

        let result = sqlx::migrate!("db/migrations")
            .run_direct(None, &mut connection, false)
            .await
            .map_err(|_| "O backup não é compatível com as migrations desta versão.".to_string());

        let closed = connection.close().await;

        result?;

        closed.map_err(|_| "Não foi possível fechar a conexão das migrations.".to_string())?;

        Ok(())
    }

    async fn change_database_key(
        &self,
        path: &Path,
        imported_key: &str,
        local_key: &str,
    ) -> Result<(), String> {
        let options =
            connection_options(path, imported_key).pragma("rekey", format!("\"x'{}'\"", local_key));

        let connection = SqliteConnection::connect_with(&options)
            .await
            .map_err(|_| {
                "Não foi possível aplicar a chave local ao banco restaurado.".to_string()
            })?;

        connection
            .close()
            .await
            .map_err(|_| "Não foi possível finalizar a troca de chave.".to_string())?;

        Ok(())
    }

    async fn find_photo_paths(
        &self,
        database_path: &Path,
        key: &str,
    ) -> Result<Vec<String>, String> {
        let mut connection =
            SqliteConnection::connect_with(&connection_options(database_path, key))
                .await
                .map_err(|_| {
                    "Não foi possível abrir o banco para consultar as fotos.".to_string()
                })?;

        let result = sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT file_path
            FROM physical_assessment_photos
            ORDER BY file_path
            "#,
        )
        .fetch_all(&mut connection)
        .await
        .map_err(|_| "Não foi possível consultar as fotos do banco.".to_string());

        let closed = connection.close().await;

        let paths = result?;

        closed.map_err(|_| "Não foi possível fechar a consulta das fotos.".to_string())?;

        Ok(paths)
    }

    async fn update_photo_path(
        &self,
        database_path: &Path,
        key: &str,
        previous_path: String,
        new_path: String,
    ) -> Result<(), String> {
        let mut connection =
            SqliteConnection::connect_with(&connection_options(database_path, key))
                .await
                .map_err(|_| "Não foi possível abrir o banco temporário.".to_string())?;

        let result = sqlx::query(
            r#"
            UPDATE physical_assessment_photos
            SET file_path = $1
            WHERE file_path = $2
            "#,
        )
        .bind(new_path)
        .bind(previous_path)
        .execute(&mut connection)
        .await
        .map_err(|_| "Não foi possível atualizar o caminho da foto restaurada.".to_string());

        let closed = connection.close().await;

        let affected = result?.rows_affected();

        closed.map_err(|_| "Não foi possível fechar o banco temporário.".to_string())?;

        if affected == 0 {
            return Err("A foto restaurada não foi encontrada no banco.".to_string());
        }

        Ok(())
    }

    async fn close_for_restore(&self, database_path: &Path, key: &str) -> Result<(), String> {
        self.close().await;

        let connection = SqliteConnection::connect_with(
            &connection_options(database_path, key),
        )
            .await
            .map_err(|_| {
                "Não foi possível preparar o banco atual. Ele não foi substituído. Reinicie o aplicativo."
                    .to_string()
            })?;

        connection.close().await.map_err(|_| {
            "Não foi possível fechar o banco atual. Reinicie o aplicativo.".to_string()
        })?;

        Ok(())
    }
}

async fn check_integrity(connection: &mut SqliteConnection) -> Result<(), String> {
    let result = sqlx::query_scalar::<_, String>("PRAGMA integrity_check")
        .fetch_all(&mut *connection)
        .await
        .map_err(|_| "Não foi possível validar a integridade do banco.".to_string())?;

    if result.len() != 1 || result[0] != "ok" {
        return Err("O banco está corrompido.".to_string());
    }

    let invalid_relationship = sqlx::query("PRAGMA foreign_key_check")
        .fetch_optional(&mut *connection)
        .await
        .map_err(|_| "Não foi possível validar os relacionamentos do banco.".to_string())?;

    if invalid_relationship.is_some() {
        return Err("O banco possui relacionamentos inconsistentes.".to_string());
    }

    Ok(())
}

fn connection_options(path: &Path, key: &str) -> SqliteConnectOptions {
    SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(false)
        .pragma("key", format!("\"x'{}'\"", key))
        .journal_mode(SqliteJournalMode::Delete)
        .foreign_keys(true)
        .with_regexp()
        .disable_statement_logging()
}
