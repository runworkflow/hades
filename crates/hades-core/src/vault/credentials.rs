use crate::key_derivation::{KdfParams, Salt, derive_key};
use crate::key_wrapping::wrap_key;
use crate::settings_authentication::sign_settings;
use crate::store::{Store, StoreError, VaultSettings};

use super::error::{VaultError, VaultResult};
use super::lifecycle::{PASSWORD_WRAP_CONTEXT, RECOVERY_WRAP_CONTEXT, unwrap_with_password};
use super::password::{check_password_strength, normalize_password};
use super::recovery_key::RecoveryKey;
use super::unlocked_vault::UnlockedVault;

pub(super) fn confirmed_settings<S: Store>(
    store: &S,
    current_password: &str,
) -> VaultResult<VaultSettings> {
    let settings = store
        .get_vault_settings()?
        .ok_or(VaultError::NotInitialized)?;
    unwrap_with_password(&settings, current_password)?;
    Ok(settings)
}

fn next_settings_version(current: u64) -> VaultResult<u64> {
    current
        .checked_add(1)
        .ok_or(VaultError::Store(StoreError::CorruptRecord))
}

fn with_authentication(
    vault: &UnlockedVault,
    settings: VaultSettings,
) -> VaultResult<VaultSettings> {
    let authentication = sign_settings(vault.vault_key(), &settings)?;
    Ok(VaultSettings {
        authentication: Some(authentication),
        ..settings
    })
}

pub fn change_password<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    current_password: &str,
    new_password: &str,
) -> VaultResult<()> {
    let settings = confirmed_settings(store, current_password)?;
    let normalized_new_password = normalize_password(new_password);
    check_password_strength(&normalized_new_password)?;

    let kdf_salt = Salt::generate();
    let kdf_params = KdfParams::default();
    let password_key = derive_key(normalized_new_password.as_bytes(), &kdf_salt, &kdf_params)?;

    let updated_settings = VaultSettings {
        settings_version: next_settings_version(settings.settings_version)?,
        kdf_salt,
        kdf_params,
        password_wrapped_key: wrap_key(&password_key, vault.vault_key(), PASSWORD_WRAP_CONTEXT)?,
        ..settings
    };
    store.put_vault_settings(&with_authentication(vault, updated_settings)?)?;
    Ok(())
}

pub fn regenerate_recovery_key<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    current_password: &str,
) -> VaultResult<RecoveryKey> {
    let settings = confirmed_settings(store, current_password)?;
    let recovery_key = RecoveryKey::generate();

    let updated_settings = VaultSettings {
        settings_version: next_settings_version(settings.settings_version)?,
        recovery_wrapped_key: wrap_key(
            recovery_key.as_secret_key(),
            vault.vault_key(),
            RECOVERY_WRAP_CONTEXT,
        )?,
        ..settings
    };
    store.put_vault_settings(&with_authentication(vault, updated_settings)?)?;
    Ok(recovery_key)
}
