use hades_core::{BiometricError, BiometricProvider, VaultError};
use serde::Serialize;
use tauri::State;
use zeroize::Zeroizing;

use crate::app_vault::AppVault;
use crate::command_error::CommandError;
use crate::macos_biometrics::MacosBiometrics;
use crate::vault_commands::{
    lock_session, lock_session_for_attempt, record_activity, record_unlock_success, throttled,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BiometricStatus {
    is_available: bool,
    is_enabled: bool,
}

fn finish_with_cancel_as_false(result: Result<(), VaultError>) -> Result<bool, CommandError> {
    match result {
        Ok(()) => Ok(true),
        Err(VaultError::Biometric(BiometricError::Cancelled)) => Ok(false),
        Err(error) => Err(error.into()),
    }
}

#[tauri::command(async)]
pub fn get_biometric_status(vault: State<'_, AppVault>) -> Result<BiometricStatus, CommandError> {
    let is_enabled = lock_session(&vault)?.is_biometric_unlock_enabled()?;
    Ok(BiometricStatus {
        is_available: MacosBiometrics.is_available(),
        is_enabled,
    })
}

#[tauri::command(async)]
pub fn enable_biometric_unlock(
    vault: State<'_, AppVault>,
    current_password: String,
) -> Result<bool, CommandError> {
    let current_password = Zeroizing::new(current_password);
    record_activity(&vault);
    let outcome = throttled(&vault, || {
        match lock_session_for_attempt(&vault)
            .enable_biometric_unlock(&current_password, &MacosBiometrics)
        {
            Ok(()) => Ok(true),
            Err(VaultError::Biometric(BiometricError::Cancelled)) => Ok(false),
            Err(error) => Err(error),
        }
    })?;
    Ok(outcome)
}

#[tauri::command(async)]
pub fn disable_biometric_unlock(vault: State<'_, AppVault>) -> Result<(), CommandError> {
    lock_session(&vault)?.disable_biometric_unlock()?;
    Ok(())
}

#[tauri::command(async)]
pub fn unlock_with_biometrics(vault: State<'_, AppVault>) -> Result<bool, CommandError> {
    record_activity(&vault);
    let result = lock_session(&vault)?.unlock_with_biometrics(&MacosBiometrics);
    let unlocked = finish_with_cancel_as_false(result)?;
    if unlocked {
        record_unlock_success(&vault);
        vault.after_unlock();
    }
    Ok(unlocked)
}
