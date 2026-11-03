use sqlx::SqlitePool;
use tauri::{AppHandle, Manager, State};

use crate::{
    dtos::backup_dto::{BackupResultDto, BackupSettingsDto, BackupSettingsRequestDto},
    services::backup_manager,
};

#[tauri::command]
pub fn get_backup_settings(app: AppHandle) -> Result<Option<BackupSettingsDto>, String> {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Não foi possível localizar a pasta do aplicativo.".to_string())?;

    backup_manager::get_settings(&data_dir)
}

#[tauri::command]
pub fn configure_backup(
    app: AppHandle,
    dto: BackupSettingsRequestDto,
) -> Result<BackupSettingsDto, String> {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Não foi possível localizar a pasta do aplicativo.".to_string())?;

    backup_manager::configure(&data_dir, dto)
}

#[tauri::command]
pub async fn create_manual_backup(
    app: AppHandle,
    pool: State<'_, SqlitePool>,
) -> Result<BackupResultDto, String> {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Não foi possível localizar a pasta do aplicativo.".to_string())?;

    backup_manager::create_manual(pool.inner(), &data_dir).await
}
