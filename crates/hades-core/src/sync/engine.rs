use crate::blobs::FileBlobStore;
use crate::store::Store;
use crate::vault::UnlockedVault;

use super::blob_sync::{pull_needed_blobs, push_missing_blobs};
use super::record_sync::sync_records;
use super::settings_sync::sync_settings;
use super::shared_folder::SharedFolder;
use super::sync_error::{SyncError, SyncResult};
use super::sync_report::SyncReport;

pub fn sync_with_folder<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    blobs: &FileBlobStore,
    folder: &SharedFolder,
    now_ms: i64,
) -> SyncResult<SyncReport> {
    let mut report = SyncReport::default();
    sync_settings(store, folder, Some(vault), &mut report)?;
    sync_records(store, folder, Some(vault), now_ms, &mut report)?;
    push_missing_blobs(blobs, folder, &mut report)?;
    pull_needed_blobs(store, vault, blobs, folder, &mut report)?;
    Ok(report)
}

pub fn join_from_folder<S: Store>(store: &S, folder: &SharedFolder) -> SyncResult<()> {
    if store.get_vault_settings()?.is_some() {
        return Err(SyncError::VaultAlreadyExists);
    }
    let remote = folder.read_settings()?.ok_or(SyncError::FolderHasNoVault)?;
    if remote.authentication.is_none() {
        return Err(SyncError::UnsignedSettings);
    }
    store.put_vault_settings(&remote.into_vault_settings(None))?;
    store.clear_sync_baselines()?;
    Ok(())
}

pub fn push_to_folder<S: Store>(
    store: &S,
    blobs: &FileBlobStore,
    folder: &SharedFolder,
) -> SyncResult<SyncReport> {
    let mut report = SyncReport::default();
    sync_settings(store, folder, None, &mut report)?;
    sync_records(store, folder, None, 0, &mut report)?;
    push_missing_blobs(blobs, folder, &mut report)?;
    Ok(report)
}
