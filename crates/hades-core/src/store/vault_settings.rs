use crate::key_derivation::{KdfParams, Salt};
use crate::key_wrapping::WrappedKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiometricSettings {
    pub key_blob: Vec<u8>,
    pub ephemeral_public_key: Vec<u8>,
    pub wrapped_key: WrappedKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultSettings {
    pub vault_id: String,
    pub settings_version: u64,
    pub created_at: i64,
    pub kdf_salt: Salt,
    pub kdf_params: KdfParams,
    pub password_wrapped_key: WrappedKey,
    pub recovery_wrapped_key: WrappedKey,
    pub biometric: Option<BiometricSettings>,
}
