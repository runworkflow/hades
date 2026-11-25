mod error;
mod key_derivation;
mod key_wrapping;
mod sealed;
mod secret_key;
mod store;
mod vault;

pub use error::{CryptoError, CryptoResult};
pub use key_derivation::{KdfParams, SALT_LENGTH, Salt, derive_key};
pub use key_wrapping::{WrappedKey, unwrap_key, wrap_key};
pub use sealed::{NONCE_LENGTH, Sealed, open, seal};
pub use secret_key::{KEY_LENGTH, SecretKey};
pub use store::{
    Envelope, EnvelopeKind, SqliteStore, Store, StoreError, StoreResult, VaultSettings,
};
pub use vault::{
    MINIMUM_PASSWORD_LENGTH, NewVault, RecoveryKey, UnlockedVault, VaultError, VaultResult,
    create_vault, unlock_with_password, unlock_with_recovery_key,
};
