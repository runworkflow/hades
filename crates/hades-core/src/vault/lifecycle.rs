use crate::key_derivation::{KdfParams, Salt, derive_key};
use crate::key_wrapping::{unwrap_key, wrap_key};
use crate::records::new_record_id;
use crate::secret_key::SecretKey;
use crate::settings_authentication::{is_settings_authentic, sign_settings};
use crate::store::{Store, VaultSettings};

use super::error::{VaultError, VaultResult};
use super::password::{check_password_strength, normalize_password};
use super::recovery_key::RecoveryKey;
use super::unlocked_vault::UnlockedVault;

pub(super) const PASSWORD_WRAP_CONTEXT: &[u8] = b"hades:vault-key:password";
pub(super) const RECOVERY_WRAP_CONTEXT: &[u8] = b"hades:vault-key:recovery";

pub struct NewVault {
    pub vault: UnlockedVault,
    pub recovery_key: RecoveryKey,
}

pub fn create_vault<S: Store>(store: &S, password: &str, created_at: i64) -> VaultResult<NewVault> {
    if store.get_vault_settings()?.is_some() {
        return Err(VaultError::AlreadyInitialized);
    }
    let normalized_password = normalize_password(password);
    check_password_strength(&normalized_password)?;

    let vault_key = SecretKey::generate();
    let recovery_key = RecoveryKey::generate();
    let kdf_salt = Salt::generate();
    let kdf_params = KdfParams::default();

    let password_key = derive_key(normalized_password.as_bytes(), &kdf_salt, &kdf_params)?;
    let unsigned_settings = VaultSettings {
        vault_id: new_record_id(),
        settings_version: 1,
        created_at,
        kdf_salt,
        kdf_params,
        password_wrapped_key: wrap_key(&password_key, &vault_key, PASSWORD_WRAP_CONTEXT)?,
        recovery_wrapped_key: wrap_key(
            recovery_key.as_secret_key(),
            &vault_key,
            RECOVERY_WRAP_CONTEXT,
        )?,
        authentication: None,
        biometric: None,
    };
    let authentication = sign_settings(&vault_key, &unsigned_settings)?;
    store.put_vault_settings(&VaultSettings {
        authentication: Some(authentication),
        ..unsigned_settings
    })?;

    Ok(NewVault {
        vault: UnlockedVault::new(vault_key),
        recovery_key,
    })
}

pub(super) fn unwrap_with_password(
    settings: &VaultSettings,
    password: &str,
) -> VaultResult<SecretKey> {
    let normalized_password = normalize_password(password);
    let password_key = derive_key(
        normalized_password.as_bytes(),
        &settings.kdf_salt,
        &settings.kdf_params,
    )?;
    unwrap_key(
        &password_key,
        &settings.password_wrapped_key,
        PASSWORD_WRAP_CONTEXT,
    )
    .map_err(|_| VaultError::IncorrectPassword)
}

pub(super) fn ensure_settings_authenticated<S: Store>(
    store: &S,
    settings: &VaultSettings,
    vault_key: &SecretKey,
) -> VaultResult<()> {
    match &settings.authentication {
        Some(authentication) => {
            if is_settings_authentic(vault_key, settings, authentication) {
                Ok(())
            } else {
                Err(VaultError::SettingsNotAuthentic)
            }
        }
        None => {
            let authentication = sign_settings(vault_key, settings)?;
            store.put_vault_settings(&VaultSettings {
                authentication: Some(authentication),
                ..settings.clone()
            })?;
            Ok(())
        }
    }
}

pub fn unlock_with_password<S: Store>(store: &S, password: &str) -> VaultResult<UnlockedVault> {
    let settings = store
        .get_vault_settings()?
        .ok_or(VaultError::NotInitialized)?;
    let vault_key = unwrap_with_password(&settings, password)?;
    ensure_settings_authenticated(store, &settings, &vault_key)?;
    Ok(UnlockedVault::new(vault_key))
}

pub fn unlock_with_recovery_key<S: Store>(
    store: &S,
    recovery_key_text: &str,
) -> VaultResult<UnlockedVault> {
    let settings = store
        .get_vault_settings()?
        .ok_or(VaultError::NotInitialized)?;
    let recovery_key = RecoveryKey::parse(recovery_key_text)?;

    let vault_key = unwrap_key(
        recovery_key.as_secret_key(),
        &settings.recovery_wrapped_key,
        RECOVERY_WRAP_CONTEXT,
    )
    .map_err(|_| VaultError::IncorrectRecoveryKey)?;
    ensure_settings_authenticated(store, &settings, &vault_key)?;
    Ok(UnlockedVault::new(vault_key))
}
