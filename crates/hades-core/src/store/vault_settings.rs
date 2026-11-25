use crate::key_derivation::{KdfParams, Salt};
use crate::key_wrapping::WrappedKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultSettings {
    pub created_at: i64,
    pub kdf_salt: Salt,
    pub kdf_params: KdfParams,
    pub password_wrapped_key: WrappedKey,
    pub recovery_wrapped_key: WrappedKey,
}
