use std::sync::PoisonError;
use std::thread;
use std::time::Duration;

use hades_core::LockReason;
use tauri::{AppHandle, Emitter, Manager};

use crate::app_vault::AppVault;
use crate::clock::current_unix_millis;

const CHECK_INTERVAL: Duration = Duration::from_secs(5);
const VAULT_LOCKED_EVENT: &str = "vault-locked";

pub fn start(app: AppHandle) {
    thread::spawn(move || {
        loop {
            thread::sleep(CHECK_INTERVAL);
            let vault = app.state::<AppVault>();
            if let Some(reason) = due_lock_reason(&vault) {
                lock_if_unlocked(&app, &vault, reason);
            }
        }
    });
}

fn due_lock_reason(vault: &AppVault) -> Option<LockReason> {
    let mut monitor = vault
        .auto_lock
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    monitor.check(current_unix_millis())
}

fn lock_if_unlocked(app: &AppHandle, vault: &AppVault, reason: LockReason) {
    let mut session = vault.session.lock().unwrap_or_else(PoisonError::into_inner);
    if session.unlocked_vault().is_none() {
        return;
    }
    session.lock();
    drop(session);
    vault.clipboard.clear_now();
    let _ = app.emit(VAULT_LOCKED_EVENT, reason.as_str());
}
