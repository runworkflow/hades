use thiserror::Error;

use crate::blobs::BlobError;
use crate::store::StoreError;
use crate::vault::VaultError;

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("could not read or write the sync folder: {0}")]
    Io(#[from] std::io::Error),
    #[error("a file in the sync folder is damaged")]
    DamagedFile,
    #[error("a file in the sync folder was written by a newer version of Hades")]
    UnsupportedFormat,
    #[error("the vault in this sync folder was saved by an older version of Hades")]
    UnsignedSettings,
    #[error("a file in the sync folder has a name Hades does not accept")]
    InvalidName,
    #[error("this sync folder belongs to a different vault")]
    DifferentVault,
    #[error("this sync folder does not contain a vault yet")]
    FolderHasNoVault,
    #[error("this Mac already has a vault, so it cannot join another one")]
    VaultAlreadyExists,
    #[error("there is no vault on this Mac to sync yet")]
    VaultNotCreated,
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Vault(#[from] VaultError),
    #[error(transparent)]
    Blob(#[from] BlobError),
}

pub type SyncResult<T> = Result<T, SyncError>;
