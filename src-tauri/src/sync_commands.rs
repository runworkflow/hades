use std::path::{Path, PathBuf};

use hades_core::{SharedFolder, Store, SyncError, join_from_folder};
use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use crate::app_vault::AppVault;
use crate::command_error::CommandError;
use crate::sync_folder_file;
use crate::sync_runner::run_if_due;
use crate::sync_state::{SyncSnapshot, SyncSummary};
use crate::vault_commands::{lock_session, record_activity};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncSummaryResponse {
    records_pushed: usize,
    records_pulled: usize,
    conflict_copies: usize,
    settings_pulled: bool,
    settings_conflict: bool,
    skipped_file_count: usize,
    rejected_record_count: usize,
    missing_blob_count: usize,
}

impl From<SyncSummary> for SyncSummaryResponse {
    fn from(summary: SyncSummary) -> Self {
        Self {
            records_pushed: summary.records_pushed,
            records_pulled: summary.records_pulled,
            conflict_copies: summary.conflict_copies,
            settings_pulled: summary.settings_pulled,
            settings_conflict: summary.settings_conflict,
            skipped_file_count: summary.skipped_file_count,
            rejected_record_count: summary.rejected_record_count,
            missing_blob_count: summary.missing_blob_count,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatusResponse {
    folder_path: Option<String>,
    is_syncing: bool,
    last_synced_at: Option<i64>,
    last_summary: Option<SyncSummaryResponse>,
    error_message: Option<String>,
}

impl From<SyncSnapshot> for SyncStatusResponse {
    fn from(snapshot: SyncSnapshot) -> Self {
        Self {
            folder_path: snapshot
                .folder
                .map(|folder| folder.to_string_lossy().into_owned()),
            is_syncing: snapshot.is_running,
            last_synced_at: snapshot.last_success_ms,
            last_summary: snapshot.last_summary.map(SyncSummaryResponse::from),
            error_message: snapshot.last_error,
        }
    }
}

fn pick_folder(app: &AppHandle, title: &str) -> Result<Option<PathBuf>, CommandError> {
    let Some(chosen) = app.dialog().file().set_title(title).blocking_pick_folder() else {
        return Ok(None);
    };
    let path = chosen
        .into_path()
        .map_err(|_| CommandError::from(SyncError::DamagedFile))?;
    Ok(Some(path))
}

fn remember_folder(vault: &AppVault, folder: &Path) -> Result<(), CommandError> {
    sync_folder_file::save(&vault.data_directory, folder)
        .map_err(|_| CommandError::from(SyncError::DamagedFile))?;
    vault.sync.set_folder(Some(folder.to_path_buf()));
    Ok(())
}

#[tauri::command(async)]
pub fn get_sync_status(vault: State<'_, AppVault>) -> Result<SyncStatusResponse, CommandError> {
    Ok(vault.sync.snapshot().into())
}

#[tauri::command(async)]
pub fn choose_sync_folder(
    app: AppHandle,
    vault: State<'_, AppVault>,
) -> Result<Option<SyncStatusResponse>, CommandError> {
    record_activity(&vault);
    let Some(folder) = pick_folder(&app, "Choose a sync folder")? else {
        return Ok(None);
    };
    let shared = SharedFolder::open(&folder)?;
    {
        let session = lock_session(&vault)?;
        let local = session.store().get_vault_settings()?;
        let remote = shared.read_settings()?;
        if let (Some(local), Some(remote)) = (local, remote)
            && local.vault_id != remote.vault_id
        {
            return Err(SyncError::DifferentVault.into());
        }
        if vault.sync.snapshot().folder.as_ref() != Some(&folder) {
            session.store().clear_sync_baselines()?;
        }
    }
    remember_folder(&vault, &folder)?;
    Ok(Some(vault.sync.snapshot().into()))
}

#[tauri::command(async)]
pub fn turn_off_sync(vault: State<'_, AppVault>) -> Result<SyncStatusResponse, CommandError> {
    record_activity(&vault);
    sync_folder_file::remove(&vault.data_directory).map_err(|_| CommandError::internal())?;
    lock_session(&vault)?.store().clear_sync_baselines()?;
    vault.sync.set_folder(None);
    Ok(vault.sync.snapshot().into())
}

#[tauri::command(async)]
pub fn sync_now(
    app: AppHandle,
    vault: State<'_, AppVault>,
) -> Result<SyncStatusResponse, CommandError> {
    record_activity(&vault);
    run_if_due(&app, &vault, true);
    Ok(vault.sync.snapshot().into())
}

#[tauri::command(async)]
pub fn join_from_sync_folder(
    app: AppHandle,
    vault: State<'_, AppVault>,
) -> Result<bool, CommandError> {
    let Some(folder) = pick_folder(&app, "Choose the folder that holds your vault")? else {
        return Ok(false);
    };
    let shared = SharedFolder::open_existing(&folder)?;
    join_from_folder(lock_session(&vault)?.store(), &shared)?;
    remember_folder(&vault, &folder)?;
    Ok(true)
}
