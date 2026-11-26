mod blobs;
mod error;
mod folders;
mod items;
mod key_derivation;
mod key_wrapping;
mod records;
mod sealed;
mod secret_key;
mod settings_authentication;
mod store;
mod sync;
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
pub use settings_authentication::{is_settings_authentic, sign_settings};
pub use store::{
    Envelope, EnvelopeKind, SqliteStore, Store, StoreError, StoreResult, SyncBaseline,
    VaultSettings,
};
pub use sync::{
    ConflictWinner, MergeDecision, RecordScan, SharedFolder, SyncError, SyncReport, SyncResult,
    SyncedSettings, decide_merge, join_from_folder, push_to_folder, sync_with_folder,
};
pub use vault::{
    AutoLockMonitor, AutoLockPolicy, BiometricEnrollment, BiometricError, BiometricProvider,
    LockReason, MINIMUM_PASSWORD_LENGTH, NewVault, RecoveryKey, UnlockThrottle, UnlockedVault,
    VaultError, VaultResult, VaultSession, VaultStatus, create_vault, unlock_with_password,
    unlock_with_recovery_key,
};
