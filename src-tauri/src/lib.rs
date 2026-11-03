use tauri::Manager;
mod db;
mod domain;
mod dtos;
mod errors;
mod handlers;
mod models;
mod repositories;
mod services;
mod utils;

#[cfg(test)]
mod tests;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let data_dir = app.path().app_local_data_dir()?;

            let pool = tauri::async_runtime::block_on(db::init(&data_dir))?;
            tauri::async_runtime::block_on(
                handlers::contract_handler::complete_expired_contracts(&pool),
            )
                .map_err(|error| {
                    std::io::Error::other(format!(
                        "Não foi possível atualizar os contratos vencidos: {error:?}"
                    ))
                })?;
            let backup_pool = pool.clone();

            app.manage(pool);

            tauri::async_runtime::spawn(async move {
                match services::backup_manager::create_automatic_if_due(&backup_pool, &data_dir)
                    .await
                {
                    Ok(Some(result)) => {
                        println!("Backup automático criado: {}", result.path);

                        if let Some(warning) = result.warning {
                            eprintln!("Aviso do backup: {warning}");
                        }
                    }

                    Ok(None) => {}

                    Err(error) => {
                        eprintln!("Erro no backup automático: {error}");
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            handlers::student_handler::find_all_students,
            handlers::student_handler::find_student_by_id,
            handlers::student_handler::insert_student,
            handlers::student_handler::update_student,
            handlers::agenda_handler::find_agenda_by_period,
            handlers::agenda_handler::insert_agenda,
            handlers::agenda_handler::update_agenda,
            handlers::agenda_handler::delete_agenda,
            handlers::user_handler::find_user_by_id,
            handlers::user_handler::exists_user,
            handlers::user_handler::insert_user,
            handlers::user_handler::update_user,
            handlers::contract_handler::find_all_contracts_by_student_id,
            handlers::contract_handler::insert_contract,
            handlers::contract_handler::update_contract,
            handlers::financial_records_handler::find_all_financial_records,
            handlers::financial_records_handler::find_financial_summary,
            handlers::financial_records_handler::find_financial_contract_options,
            handlers::financial_records_handler::insert_financial_record,
            handlers::financial_records_handler::update_financial_record,
            handlers::physical_assessment_handler::find_all_physical_assessments_by_student_id,
            handlers::physical_assessment_handler::insert_physical_assessment,
            handlers::physical_assessment_handler::update_physical_assessment,
            handlers::physical_assessment_handler::delete_physical_assessment,
            handlers::financial_records_handler::generate_monthly_charges,
            handlers::backup_settings_handler::get_backup_settings,
            handlers::backup_settings_handler::configure_backup,
            handlers::backup_settings_handler::create_manual_backup,
            handlers::backup_handler::restore_backup,
            handlers::backup_handler::restart_after_restore,
            handlers::physical_assessment_photos_handler::find_all_physical_assessment_photos,
            handlers::physical_assessment_photos_handler::find_physical_assessment_photo_by_id,
            handlers::physical_assessment_photos_handler::insert_physical_assessment_photo,
            handlers::physical_assessment_photos_handler::update_physical_assessment_photo,
            handlers::physical_assessment_photos_handler::delete_physical_assessment_photo,
            handlers::physical_assessment_photos_handler::read_physical_assessment_photo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
