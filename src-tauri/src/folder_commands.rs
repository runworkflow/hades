use hades_core::Folder;
use serde::Serialize;
use tauri::State;

use crate::app_vault::AppVault;
use crate::clock::current_unix_millis;
use crate::command_error::CommandError;
use crate::vault_commands::lock_session;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderResponse {
    id: String,
    name: String,
    icon_name: String,
}

impl From<Folder> for FolderResponse {
    fn from(folder: Folder) -> Self {
        Self {
            id: folder.id,
            name: folder.name,
            icon_name: folder.icon,
        }
    }
}

#[tauri::command(async)]
pub fn list_folders(vault: State<'_, AppVault>) -> Result<Vec<FolderResponse>, CommandError> {
    let folders = lock_session(&vault)?.list_folders()?;
    Ok(folders.into_iter().map(FolderResponse::from).collect())
}

#[tauri::command(async)]
pub fn create_folder(
    vault: State<'_, AppVault>,
    name: String,
    icon_name: String,
) -> Result<FolderResponse, CommandError> {
    let folder = lock_session(&vault)?.create_folder(&name, &icon_name, current_unix_millis())?;
    Ok(folder.into())
}
