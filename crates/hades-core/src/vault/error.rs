use thiserror::Error;

use crate::error::CryptoError;
use crate::store::StoreError;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("the password must be at least {minimum_length} characters")]
    WeakPassword { minimum_length: usize },
    #[error("the vault has not been created yet")]
    NotInitialized,
    #[error("a vault already exists here")]
    AlreadyInitialized,
    #[error("incorrect password")]
    IncorrectPassword,
    #[error("the recovery key is not in a valid format")]
    InvalidRecoveryKey,
    #[error("incorrect recovery key")]
    IncorrectRecoveryKey,
    #[error("the vault is locked")]
    VaultLocked,
    #[error("the name must be between 1 and {maximum_length} characters")]
    InvalidName { maximum_length: usize },
    #[error("the icon name is not valid")]
    InvalidIcon,
    #[error(transparent)]
    Crypto(#[from] CryptoError),
    #[error(transparent)]
    Store(#[from] StoreError),
}

pub type VaultResult<T> = Result<T, VaultError>;
