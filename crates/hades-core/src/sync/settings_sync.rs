use std::cmp::Ordering;

use crate::store::Store;

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

pub(super) fn sync_settings<S: Store>(
    store: &S,
    folder: &SharedFolder,
    should_pull: bool,
    report: &mut SyncReport,
) -> SyncResult<()> {
    let local_settings = store
        .get_vault_settings()?
        .ok_or(SyncError::VaultNotCreated)?;
    let local = SyncedSettings::from_vault_settings(&local_settings);
    let Some(remote) = folder.read_settings()? else {
        folder.write_settings(&local)?;
        report.settings_pushed = true;
        return Ok(());
    };
    if remote.vault_id != local.vault_id {
        return Err(SyncError::DifferentVault);
    }
    if remote == local {
        return Ok(());
    }

    let (direction, is_conflict) = choose_direction(&local, &remote);
    match direction {
        SettingsDirection::Push => {
            folder.write_settings(&local)?;
            report.settings_pushed = true;
        }
        SettingsDirection::Pull if should_pull => {
            store.put_vault_settings(&remote.into_vault_settings(local_settings.biometric))?;
            report.settings_pulled = true;
        }
        SettingsDirection::Pull => return Ok(()),
    }
    report.settings_conflict = is_conflict;
    Ok(())
}
