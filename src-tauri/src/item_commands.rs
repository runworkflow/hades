use std::collections::HashMap;
use std::time::Duration;

use hades_core::{ItemDetail, ItemField, ItemKind, ItemSummary, ItemUpdate, NewItem, VaultError};
use serde::Serialize;
use tauri::State;

use crate::app_vault::AppVault;
use crate::clock::current_unix_millis;
use crate::command_error::CommandError;
use crate::vault_commands::lock_session;

const CLIPBOARD_CLEAR_AFTER: Duration = Duration::from_secs(30);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSummaryResponse {
    id: String,
    name: String,
    kind: &'static str,
    folder_id: String,
    modified_at: i64,
}

impl From<ItemSummary> for ItemSummaryResponse {
    fn from(summary: ItemSummary) -> Self {
        Self {
            id: summary.id,
            name: summary.name,
            kind: summary.kind.as_str(),
            folder_id: summary.folder_id,
            modified_at: summary.modified_at,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemFieldResponse {
    key: String,
    is_secret: bool,
    has_value: bool,
    value: Option<String>,
}

impl From<ItemField> for ItemFieldResponse {
    fn from(field: ItemField) -> Self {
        Self {
            key: field.key,
            is_secret: field.is_secret,
            has_value: field.has_value,
            value: field.value,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDetailResponse {
    #[serde(flatten)]
    summary: ItemSummaryResponse,
    fields: Vec<ItemFieldResponse>,
}

impl From<ItemDetail> for ItemDetailResponse {
    fn from(detail: ItemDetail) -> Self {
        Self {
            summary: detail.summary.into(),
            fields: detail
                .fields
                .into_iter()
                .map(ItemFieldResponse::from)
                .collect(),
        }
    }
}

#[tauri::command(async)]
pub fn list_items(vault: State<'_, AppVault>) -> Result<Vec<ItemSummaryResponse>, CommandError> {
    let items = lock_session(&vault)?.list_items()?;
    Ok(items.into_iter().map(ItemSummaryResponse::from).collect())
}

#[tauri::command(async)]
pub fn get_item(
    vault: State<'_, AppVault>,
    item_id: String,
) -> Result<ItemDetailResponse, CommandError> {
    Ok(lock_session(&vault)?.get_item(&item_id)?.into())
}

#[tauri::command(async)]
pub fn create_item(
    vault: State<'_, AppVault>,
    folder_id: String,
    kind: String,
    name: String,
    fields: HashMap<String, String>,
) -> Result<ItemSummaryResponse, CommandError> {
    let new_item = NewItem {
        folder_id,
        kind: ItemKind::parse(&kind).ok_or(VaultError::InvalidItemKind)?,
        name,
        fields: fields.into_iter().collect(),
    };
    let summary = lock_session(&vault)?.create_item(new_item, current_unix_millis())?;
    Ok(summary.into())
}

#[tauri::command(async)]
pub fn update_item(
    vault: State<'_, AppVault>,
    item_id: String,
    folder_id: String,
    name: String,
    fields: HashMap<String, String>,
) -> Result<ItemSummaryResponse, CommandError> {
    let update = ItemUpdate {
        folder_id,
        name,
        fields: fields.into_iter().collect(),
    };
    let summary = lock_session(&vault)?.update_item(&item_id, update, current_unix_millis())?;
    Ok(summary.into())
}

#[tauri::command(async)]
pub fn delete_item(vault: State<'_, AppVault>, item_id: String) -> Result<(), CommandError> {
    lock_session(&vault)?.delete_item(&item_id, current_unix_millis())?;
    vault.remove_orphan_blobs();
    Ok(())
}

#[tauri::command(async)]
pub fn delete_items(
    vault: State<'_, AppVault>,
    item_ids: Vec<String>,
) -> Result<usize, CommandError> {
    let deleted_count = lock_session(&vault)?.delete_items(&item_ids, current_unix_millis())?;
    vault.remove_orphan_blobs();
    Ok(deleted_count)
}

#[tauri::command(async)]
pub fn move_items(
    vault: State<'_, AppVault>,
    item_ids: Vec<String>,
    folder_id: String,
) -> Result<usize, CommandError> {
    Ok(lock_session(&vault)?.move_items(&item_ids, &folder_id, current_unix_millis())?)
}

#[tauri::command(async)]
pub fn copy_item_field(
    vault: State<'_, AppVault>,
    item_id: String,
    key: String,
) -> Result<(), CommandError> {
    let value = lock_session(&vault)?.read_item_field(&item_id, &key)?;
    vault
        .clipboard
        .copy(value, CLIPBOARD_CLEAR_AFTER)
        .map_err(|_| CommandError::internal())
}

#[tauri::command(async)]
pub fn reveal_item_field(
    vault: State<'_, AppVault>,
    item_id: String,
    key: String,
) -> Result<String, CommandError> {
    let value = lock_session(&vault)?.reveal_item_field(&item_id, &key)?;
    Ok(value.to_string())
}
