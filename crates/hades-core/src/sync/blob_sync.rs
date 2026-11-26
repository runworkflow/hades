use std::collections::BTreeSet;

use crate::blobs::FileBlobStore;
use crate::items::referenced_blob_ids;
use crate::store::Store;
use crate::vault::UnlockedVault;

use super::shared_folder::SharedFolder;
use super::sync_error::{SyncError, SyncResult};
use super::sync_report::SyncReport;

pub(super) fn push_missing_blobs(
    local: &FileBlobStore,
    folder: &SharedFolder,
    report: &mut SyncReport,
) -> SyncResult<()> {
    let remote_ids: BTreeSet<String> = folder.list_blob_ids()?.into_iter().collect();
    for id in local.list_ids()? {
        if !remote_ids.contains(&id) {
            match folder.push_blob(local, &id) {
                Ok(()) => report.blobs_pushed += 1,
                Err(SyncError::Io(_) | SyncError::DamagedFile) => {
                    report.skipped_files.push(format!("{id}.blob"));
                }
                Err(error) => return Err(error),
            }
        }
    }
    Ok(())
}

pub(super) fn pull_needed_blobs<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    local: &FileBlobStore,
    folder: &SharedFolder,
    report: &mut SyncReport,
) -> SyncResult<()> {
    let local_ids: BTreeSet<String> = local.list_ids()?.into_iter().collect();
    let remote_ids: BTreeSet<String> = folder.list_blob_ids()?.into_iter().collect();
    for id in referenced_blob_ids(store, vault)? {
        if local_ids.contains(&id) {
            continue;
        }
        if remote_ids.contains(&id) && folder.pull_blob(local, &id).is_ok() {
            report.blobs_pulled += 1;
        } else {
            report.blobs_missing += 1;
        }
    }
    Ok(())
}
