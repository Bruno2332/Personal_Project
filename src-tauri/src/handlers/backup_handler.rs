use sqlx::SqlitePool;
use std::path::Path;
use tauri::{AppHandle, Manager, State};

use crate::{
    dtos::backup_dto::RestoreBackupResponseDto, services::restore_service, utils::backup_lock,
};

#[tauri::command]
pub async fn restore_backup(
    app: AppHandle,
    pool: State<'_, SqlitePool>,
    path: String,
    password: String,
) -> Result<RestoreBackupResponseDto, String> {
    let _guard = backup_lock::acquire()?;

    let data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Não foi possível localizar a pasta do aplicativo.".to_string())?;

    let result =
        restore_service::restore_backup(pool.inner(), &data_dir, Path::new(&path), password).await;

    match result {
        Ok(previous_database_path) => Ok(RestoreBackupResponseDto {
            restored: true,
            restart_required: true,
            previous_database_path: Some(previous_database_path),
            error: None,
        }),

        Err(error) if pool.is_closed() => Ok(RestoreBackupResponseDto {
            restored: false,
            restart_required: true,
            previous_database_path: None,
            error: Some(error),
        }),

        Err(error) => Err(error),
    }
}

#[tauri::command]
pub fn restart_after_restore(app: AppHandle, pool: State<'_, SqlitePool>) -> Result<(), String> {
    let _guard = backup_lock::acquire()?;

    if !pool.is_closed() {
        return Err("Não existe uma restauração aguardando reinício.".to_string());
    }

    app.request_restart();

    Ok(())
}
