mod app_vault;
mod auto_lock_runner;
mod clock;
mod command_error;
mod vault_commands;

use tauri::Manager;

use app_vault::AppVault;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_directory = app.path().app_data_dir()?;
            app.manage(AppVault::open(&data_directory)?);
            auto_lock_runner::start(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            vault_commands::get_vault_status,
            vault_commands::create_vault,
            vault_commands::unlock_with_password,
            vault_commands::unlock_with_recovery_key,
            vault_commands::lock_vault,
            vault_commands::report_activity,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
