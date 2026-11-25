const APP_COMMANDS: &[&str] = &[
    "get_vault_status",
    "create_vault",
    "unlock_with_password",
    "unlock_with_recovery_key",
    "lock_vault",
    "report_activity",
    "list_folders",
    "create_folder",
    "update_folder",
    "delete_folder",
    "list_items",
    "get_item",
    "create_item",
    "reveal_item_field",
    "copy_item_field",
    "update_item",
    "delete_item",
    "delete_items",
    "move_items",
];

fn main() {
    let app_manifest = tauri_build::AppManifest::new().commands(APP_COMMANDS);
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(app_manifest))
        .expect("failed to run the Tauri build step");
}
