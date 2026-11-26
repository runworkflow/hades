use crate::key_derivation::{KdfParams, Salt, derive_key};
use crate::key_wrapping::wrap_key;
use crate::store::{Store, VaultSettings};

use super::error::{VaultError, VaultResult};
use super::lifecycle::{PASSWORD_WRAP_CONTEXT, RECOVERY_WRAP_CONTEXT, unwrap_with_password};
use super::password::{check_password_strength, normalize_password};
use super::recovery_key::RecoveryKey;
use super::unlocked_vault::UnlockedVault;

fn confirmed_settings<S: Store>(store: &S, current_password: &str) -> VaultResult<VaultSettings> {
    let settings = store
        .get_vault_settings()?
        .ok_or(VaultError::NotInitialized)?;
    unwrap_with_password(&settings, current_password)?;
    Ok(settings)
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

    store.put_vault_settings(&VaultSettings {
        kdf_salt,
        kdf_params,
        password_wrapped_key: wrap_key(&password_key, vault.vault_key(), PASSWORD_WRAP_CONTEXT)?,
        ..settings
    })?;
    Ok(())
}

pub fn regenerate_recovery_key<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    current_password: &str,
) -> VaultResult<RecoveryKey> {
    let settings = confirmed_settings(store, current_password)?;
    let recovery_key = RecoveryKey::generate();

    store.put_vault_settings(&VaultSettings {
        recovery_wrapped_key: wrap_key(
            recovery_key.as_secret_key(),
            vault.vault_key(),
            RECOVERY_WRAP_CONTEXT,
        )?,
        ..settings
    })?;
    Ok(recovery_key)
}
