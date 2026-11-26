use serde::Serialize;
use tauri::State;
use zeroize::Zeroizing;

use crate::app_vault::AppVault;
use crate::command_error::CommandError;
use crate::vault_commands::{lock_session, record_activity};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRecoveryKey {
    recovery_key: String,
}

#[tauri::command(async)]
pub fn change_password(
    vault: State<'_, AppVault>,
    current_password: String,
    new_password: String,
) -> Result<(), CommandError> {
    let current_password = Zeroizing::new(current_password);
    let new_password = Zeroizing::new(new_password);
    record_activity(&vault);
    lock_session(&vault)?.change_password(&current_password, &new_password)?;
    Ok(())
}

#[tauri::command(async)]
pub fn create_new_recovery_key(
    vault: State<'_, AppVault>,
    current_password: String,
) -> Result<NewRecoveryKey, CommandError> {
    let current_password = Zeroizing::new(current_password);
    record_activity(&vault);
    let recovery_key = lock_session(&vault)?.regenerate_recovery_key(&current_password)?;
    Ok(NewRecoveryKey {
        recovery_key: recovery_key.to_display_text().to_string(),
    })
}
