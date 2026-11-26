use std::sync::{MutexGuard, PoisonError};

use hades_core::{SqliteStore, VaultError, VaultSession, VaultStatus};
use serde::Serialize;
use tauri::State;
use zeroize::Zeroizing;

use crate::app_vault::AppVault;
use crate::clock::current_unix_millis;
use crate::command_error::CommandError;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VaultStatusResponse {
    NotCreated,
    Locked,
    Unlocked,
}

impl From<VaultStatus> for VaultStatusResponse {
    fn from(status: VaultStatus) -> Self {
        match status {
            VaultStatus::NotCreated => Self::NotCreated,
            VaultStatus::Locked => Self::Locked,
            VaultStatus::Unlocked => Self::Unlocked,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedVault {
    recovery_key: String,
}

pub fn lock_session_for_attempt(vault: &AppVault) -> MutexGuard<'_, VaultSession<SqliteStore>> {
    vault.session.lock().unwrap_or_else(PoisonError::into_inner)
}

pub fn throttled<T>(
    vault: &AppVault,
    attempt: impl FnOnce() -> Result<T, VaultError>,
) -> Result<T, CommandError> {
    if let Ok(throttle) = vault.unlock_throttle.lock()
        && let Err(remaining_ms) = throttle.check(current_unix_millis())
    {
        return Err(VaultError::TooManyAttempts {
            retry_after_seconds: remaining_ms.unsigned_abs().div_ceil(1000),
        }
        .into());
    }

    let result = attempt();
    if let Ok(mut throttle) = vault.unlock_throttle.lock() {
        match &result {
            Ok(_) => throttle.record_success(),
            Err(VaultError::IncorrectPassword | VaultError::IncorrectRecoveryKey) => {
                throttle.record_failure(current_unix_millis());
            }
            Err(_) => {}
        }
    }
    result.map_err(CommandError::from)
}

pub fn record_unlock_success(vault: &AppVault) {
    if let Ok(mut throttle) = vault.unlock_throttle.lock() {
        throttle.record_success();
    }
}

pub fn lock_session(
    vault: &AppVault,
) -> Result<MutexGuard<'_, VaultSession<SqliteStore>>, CommandError> {
    vault.session.lock().map_err(|_| CommandError::internal())
}

pub fn record_activity(vault: &AppVault) {
    if let Ok(mut monitor) = vault.auto_lock.lock() {
        monitor.record_activity(current_unix_millis());
    }
}

#[tauri::command(async)]
pub fn get_vault_status(vault: State<'_, AppVault>) -> Result<VaultStatusResponse, CommandError> {
    Ok(lock_session(&vault)?.status()?.into())
}

#[tauri::command(async)]
pub fn create_vault(
    vault: State<'_, AppVault>,
    password: String,
) -> Result<CreatedVault, CommandError> {
    let password = Zeroizing::new(password);
    record_activity(&vault);
    let recovery_key = lock_session(&vault)?.create(&password, current_unix_millis())?;
    Ok(CreatedVault {
        recovery_key: recovery_key.to_display_text().to_string(),
    })
}

#[tauri::command(async)]
pub fn unlock_with_password(
    vault: State<'_, AppVault>,
    password: String,
) -> Result<(), CommandError> {
    let password = Zeroizing::new(password);
    record_activity(&vault);
    throttled(&vault, || {
        lock_session_for_attempt(&vault).unlock_with_password(&password)
    })?;
    vault.remove_orphan_blobs();
    Ok(())
}

#[tauri::command(async)]
pub fn unlock_with_recovery_key(
    vault: State<'_, AppVault>,
    recovery_key: String,
) -> Result<(), CommandError> {
    let recovery_key = Zeroizing::new(recovery_key);
    record_activity(&vault);
    throttled(&vault, || {
        lock_session_for_attempt(&vault).unlock_with_recovery_key(&recovery_key)
    })?;
    vault.remove_orphan_blobs();
    Ok(())
}

#[tauri::command(async)]
pub fn lock_vault(vault: State<'_, AppVault>) -> Result<(), CommandError> {
    lock_session(&vault)?.lock();
    vault.clipboard.clear_now();
    Ok(())
}

#[tauri::command(async)]
pub fn report_activity(vault: State<'_, AppVault>) -> Result<(), CommandError> {
    record_activity(&vault);
    Ok(())
}
