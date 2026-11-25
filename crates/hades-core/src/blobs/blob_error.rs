use thiserror::Error;

use crate::error::CryptoError;

#[derive(Debug, Error)]
pub enum BlobError {
    #[error("could not read or write the file: {0}")]
    Io(#[from] std::io::Error),
    #[error("the file is larger than the allowed {maximum_bytes} bytes")]
    TooLarge { maximum_bytes: u64 },
    #[error("the stored file is damaged or has been changed")]
    Corrupt,
    #[error("the file id is not valid")]
    InvalidId,
    #[error("a file with this id already exists")]
    AlreadyExists,
    #[error(transparent)]
    Crypto(#[from] CryptoError),
}

pub type BlobResult<T> = Result<T, BlobError>;
