use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CryptoError {
    #[error("invalid key derivation settings")]
    InvalidKdfParams,
    #[error("key derivation failed")]
    KeyDerivationFailed,
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("decryption failed: wrong key, wrong context or tampered data")]
    DecryptionFailed,
    #[error("unexpected key length")]
    InvalidKeyLength,
}

pub type CryptoResult<T> = Result<T, CryptoError>;
