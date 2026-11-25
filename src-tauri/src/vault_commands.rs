use std::sync::MutexGuard;
use std::time::{SystemTime, UNIX_EPOCH};

use hades_core::{SqliteStore, VaultSession, VaultStatus};
use serde::Serialize;
use tauri::State;
use zeroize::Zeroizing;

use crate::app_vault::AppVault;
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

fn lock_session(
    vault: &AppVault,
) -> Result<MutexGuard<'_, VaultSession<SqliteStore>>, CommandError> {
    vault.session.lock().map_err(|_| CommandError::internal())
}

fn current_unix_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis() as i64)
        .unwrap_or(0)
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
    lock_session(&vault)?.unlock_with_password(&password)?;
    Ok(())
}

#[tauri::command(async)]
pub fn unlock_with_recovery_key(
    vault: State<'_, AppVault>,
    recovery_key: String,
) -> Result<(), CommandError> {
    let recovery_key = Zeroizing::new(recovery_key);
    lock_session(&vault)?.unlock_with_recovery_key(&recovery_key)?;
    Ok(())
}

#[tauri::command(async)]
pub fn lock_vault(vault: State<'_, AppVault>) -> Result<(), CommandError> {
    lock_session(&vault)?.lock();
    Ok(())
}
