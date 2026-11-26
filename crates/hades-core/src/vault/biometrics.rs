use thiserror::Error;

use crate::key_wrapping::{unwrap_key, wrap_key};
use crate::secret_key::SecretKey;
use crate::store::{BiometricSettings, Store, StoreResult, VaultSettings};

use super::credentials::confirmed_settings;
use super::error::{VaultError, VaultResult};
use super::lifecycle::ensure_settings_authenticated;
use super::unlocked_vault::UnlockedVault;

const BIOMETRIC_WRAP_CONTEXT: &[u8] = b"hades:vault-key:biometric";
const ENABLE_REASON: &str = "Turn on Touch ID for Hades";
const UNLOCK_REASON: &str = "Unlock Hades";

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum BiometricError {
    #[error("Touch ID is not available on this Mac")]
    Unavailable,
    #[error("Touch ID was cancelled")]
    Cancelled,
    #[error("Touch ID no longer matches this vault")]
    KeyChanged,
    #[error("Touch ID did not work")]
    Failed,
}

pub struct BiometricEnrollment {
    pub key_blob: Vec<u8>,
    pub ephemeral_public_key: Vec<u8>,
    pub wrapping_key: SecretKey,
}

pub trait BiometricProvider {
    fn is_available(&self) -> bool;
    fn enroll(&self, reason: &str) -> Result<BiometricEnrollment, BiometricError>;
    fn derive_key(
        &self,
        key_blob: &[u8],
        ephemeral_public_key: &[u8],
        reason: &str,
    ) -> Result<SecretKey, BiometricError>;
}

pub fn is_biometric_unlock_enabled<S: Store>(store: &S) -> StoreResult<bool> {
    Ok(store
        .get_vault_settings()?
        .is_some_and(|settings| settings.biometric.is_some()))
}

pub fn enable_biometric_unlock<S: Store, P: BiometricProvider>(
    store: &S,
    vault: &UnlockedVault,
    current_password: &str,
    provider: &P,
) -> VaultResult<()> {
    if !provider.is_available() {
        return Err(BiometricError::Unavailable.into());
    }
    let settings = confirmed_settings(store, current_password)?;
    let enrollment = provider.enroll(ENABLE_REASON)?;
    let wrapped_key = wrap_key(
        &enrollment.wrapping_key,
        vault.vault_key(),
        BIOMETRIC_WRAP_CONTEXT,
    )?;

    store.put_vault_settings(&VaultSettings {
        biometric: Some(BiometricSettings {
            key_blob: enrollment.key_blob,
            ephemeral_public_key: enrollment.ephemeral_public_key,
            wrapped_key,
        }),
        ..settings
    })?;
    Ok(())
}

pub fn disable_biometric_unlock<S: Store>(store: &S) -> VaultResult<()> {
    let settings = store
        .get_vault_settings()?
        .ok_or(VaultError::NotInitialized)?;
    store.put_vault_settings(&VaultSettings {
        biometric: None,
        ..settings
    })?;
    Ok(())
}

pub fn unlock_with_biometrics<S: Store, P: BiometricProvider>(
    store: &S,
    provider: &P,
) -> VaultResult<UnlockedVault> {
    let settings = store
        .get_vault_settings()?
        .ok_or(VaultError::NotInitialized)?;
    let biometric = settings
        .biometric
        .as_ref()
        .ok_or(VaultError::BiometricsNotEnabled)?;

    let derived_key = provider.derive_key(
        &biometric.key_blob,
        &biometric.ephemeral_public_key,
        UNLOCK_REASON,
    )?;
    let vault_key = unwrap_key(&derived_key, &biometric.wrapped_key, BIOMETRIC_WRAP_CONTEXT)
        .map_err(|_| BiometricError::KeyChanged)?;
    ensure_settings_authenticated(store, &settings, &vault_key)?;
    Ok(UnlockedVault::new(vault_key))
}
