use std::path::Path;
use std::sync::{PoisonError, TryLockError};
use std::thread;
use std::time::Duration;

use hades_core::{SharedFolder, VaultError, push_to_folder, sync_with_folder};
use tauri::{AppHandle, Emitter, Manager};

use crate::app_vault::AppVault;
use crate::clock::current_unix_millis;
use crate::command_error::CommandError;
use crate::sync_state::SyncSummary;

const CHECK_INTERVAL: Duration = Duration::from_secs(2);
const VAULT_SYNCED_EVENT: &str = "vault-synced";

#[derive(Clone, Copy, PartialEq, Eq)]
enum SyncMode {
    WhileUnlocked,
    PushOnly,
}

pub fn start(app: AppHandle) {
    thread::spawn(move || {
        loop {
            thread::sleep(CHECK_INTERVAL);
            let vault = app.state::<AppVault>();
            run_if_due(&app, &vault, false);
        }
    });
}

fn is_vault_unlocked(vault: &AppVault) -> bool {
    match vault.session.try_lock() {
        Ok(session) => session.unlocked_vault().is_some(),
        Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner().unlocked_vault().is_some(),
        Err(TryLockError::WouldBlock) => false,
    }
}

pub fn run_if_due(app: &AppHandle, vault: &AppVault, is_forced: bool) {
    if !is_forced && !is_vault_unlocked(vault) {
        return;
    }
    let Some(folder) = vault.sync.begin(current_unix_millis(), is_forced) else {
        return;
    };
    let outcome = sync_once(vault, &folder, SyncMode::WhileUnlocked);
    finish_run(app, vault, outcome);
}

pub fn push_on_lock(vault: &AppVault) {
    let Some(folder) = vault.sync.begin(current_unix_millis(), true) else {
        return;
    };
    let outcome = sync_once(vault, &folder, SyncMode::PushOnly);
    vault.sync.finish(
        current_unix_millis(),
        outcome.map_err(|error| error.message().to_string()),
    );
}

fn finish_run(app: &AppHandle, vault: &AppVault, outcome: Result<SyncSummary, CommandError>) {
    if let Ok(summary) = &outcome
        && summary.has_local_changes()
    {
        let _ = app.emit(VAULT_SYNCED_EVENT, ());
    }
    vault.sync.finish(
        current_unix_millis(),
        outcome.map_err(|error| error.message().to_string()),
    );
}

fn sync_once(vault: &AppVault, folder: &Path, mode: SyncMode) -> Result<SyncSummary, CommandError> {
    let shared = SharedFolder::open(folder)?;
    let session = vault.session.lock().unwrap_or_else(PoisonError::into_inner);
    let now_ms = current_unix_millis();
    let report = match (mode, session.unlocked_vault()) {
        (SyncMode::WhileUnlocked, Some(unlocked)) => {
            sync_with_folder(session.store(), unlocked, &vault.blobs, &shared, now_ms)?
        }
        (SyncMode::WhileUnlocked, None) => return Err(VaultError::VaultLocked.into()),
        (SyncMode::PushOnly, _) => push_to_folder(session.store(), &vault.blobs, &shared)?,
    };
    Ok(SyncSummary::from_report(&report))
}
