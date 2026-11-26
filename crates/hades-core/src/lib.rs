mod blobs;
mod error;
mod folders;
mod items;
mod key_derivation;
mod key_wrapping;
mod records;
mod sealed;
mod secret_key;
mod store;
mod vault;

pub use blobs::{
    BlobError, BlobResult, CHUNK_SIZE, FileBlobStore, MAX_FILE_SIZE, decrypt_stream, encrypt_stream,
};
pub use error::{CryptoError, CryptoResult};
pub use folders::{Folder, MAX_FOLDER_NAME_LENGTH, MAX_ICON_NAME_LENGTH};
pub use items::{
    ItemDetail, ItemField, ItemKind, ItemSummary, ItemUpdate, MAX_FIELD_LENGTH,
    MAX_ITEM_NAME_LENGTH, NewItem, clean_file_name,
};
pub use key_derivation::{KdfParams, SALT_LENGTH, Salt, derive_key};
pub use key_wrapping::{WrappedKey, unwrap_key, wrap_key};
pub use sealed::{NONCE_LENGTH, Sealed, open, seal};
pub use secret_key::{KEY_LENGTH, SecretKey};
pub use store::{
    Envelope, EnvelopeKind, SqliteStore, Store, StoreError, StoreResult, VaultSettings,
};
pub use vault::{
    AutoLockMonitor, AutoLockPolicy, LockReason, MINIMUM_PASSWORD_LENGTH, NewVault, RecoveryKey,
    UnlockedVault, VaultError, VaultResult, VaultSession, VaultStatus, create_vault,
    unlock_with_password, unlock_with_recovery_key,
};
