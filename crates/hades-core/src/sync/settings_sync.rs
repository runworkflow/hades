use std::cmp::Ordering;

use crate::settings_authentication::is_settings_authentic;
use crate::store::Store;
use crate::vault::UnlockedVault;

use super::shared_folder::SharedFolder;
use super::sync_error::{SyncError, SyncResult};
use super::sync_report::SyncReport;
use super::synced_settings::SyncedSettings;

fn tie_break_key(settings: &SyncedSettings) -> (&[u8], &[u8], &[u8]) {
    (
        &settings.password_wrapped_key.0.ciphertext,
        &settings.recovery_wrapped_key.0.ciphertext,
        settings.kdf_salt.as_bytes(),
    )
}

enum SettingsDirection {
    Push,
    Pull,
}

fn choose_direction(local: &SyncedSettings, remote: &SyncedSettings) -> (SettingsDirection, bool) {
    match local.settings_version.cmp(&remote.settings_version) {
        Ordering::Greater => (SettingsDirection::Push, false),
        Ordering::Less => (SettingsDirection::Pull, false),
        Ordering::Equal if tie_break_key(local) > tie_break_key(remote) => {
            (SettingsDirection::Push, true)
        }
        Ordering::Equal => (SettingsDirection::Pull, true),
    }
}

fn is_remote_authentic(vault: &UnlockedVault, remote: &SyncedSettings) -> bool {
    let Some(authentication) = &remote.authentication else {
        return false;
    };
    let candidate = remote.clone().into_vault_settings(None);
    is_settings_authentic(vault.vault_key(), &candidate, authentication)
}

fn push_if_signed(
    folder: &SharedFolder,
    local: &SyncedSettings,
    report: &mut SyncReport,
) -> SyncResult<()> {
    if local.authentication.is_some() {
        folder.write_settings(local)?;
        report.settings_pushed = true;
    }
    Ok(())
}

pub(super) fn sync_settings<S: Store>(
    store: &S,
    folder: &SharedFolder,
    vault: Option<&UnlockedVault>,
    report: &mut SyncReport,
) -> SyncResult<()> {
    let local_settings = store
        .get_vault_settings()?
        .ok_or(SyncError::VaultNotCreated)?;
    let local = SyncedSettings::from_vault_settings(&local_settings);
    let Some(remote) = folder.read_settings()? else {
        return push_if_signed(folder, &local, report);
    };
    if remote.vault_id != local.vault_id {
        return Err(SyncError::DifferentVault);
    }
    if remote == local {
        return Ok(());
    }
    if vault.is_some_and(|vault| !is_remote_authentic(vault, &remote)) {
        report.settings_rejected = true;
        return push_if_signed(folder, &local, report);
    }

    let (direction, is_conflict) = choose_direction(&local, &remote);
    match direction {
        SettingsDirection::Push => push_if_signed(folder, &local, report)?,
        SettingsDirection::Pull if vault.is_some() => {
            store.put_vault_settings(&remote.into_vault_settings(local_settings.biometric))?;
            report.settings_pulled = true;
        }
        SettingsDirection::Pull => return Ok(()),
    }
    report.settings_conflict = is_conflict;
    Ok(())
}
