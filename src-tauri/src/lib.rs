mod app_vault;
mod atomic_file;
mod auto_lock_runner;
mod biometric_commands;
mod clock;
mod command_error;
mod file_commands;
mod folder_commands;
mod item_commands;
mod macos_biometrics;
mod macos_system_events;
mod secret_clipboard;
mod security_commands;
mod vault_commands;

use tauri::Manager;

use app_vault::AppVault;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_directory = app.path().app_data_dir()?;
            app.manage(AppVault::open(&data_directory)?);
            auto_lock_runner::start(app.handle().clone());
            macos_system_events::start(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            vault_commands::get_vault_status,
            vault_commands::create_vault,
            vault_commands::unlock_with_password,
            vault_commands::unlock_with_recovery_key,
            vault_commands::lock_vault,
            vault_commands::report_activity,
            folder_commands::list_folders,
            folder_commands::create_folder,
            folder_commands::update_folder,
            folder_commands::delete_folder,
            item_commands::list_items,
            item_commands::get_item,
            item_commands::create_item,
            item_commands::reveal_item_field,
            item_commands::copy_item_field,
            item_commands::update_item,
            item_commands::delete_item,
            item_commands::delete_items,
            item_commands::move_items,
            file_commands::import_file,
            file_commands::export_file,
            security_commands::change_password,
            security_commands::create_new_recovery_key,
            biometric_commands::get_biometric_status,
            biometric_commands::enable_biometric_unlock,
            biometric_commands::disable_biometric_unlock,
            biometric_commands::unlock_with_biometrics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
