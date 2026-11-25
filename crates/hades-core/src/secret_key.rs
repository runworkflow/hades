use std::fmt;

use chacha20poly1305::aead::Generate;
use zeroize::ZeroizeOnDrop;

use crate::error::{CryptoError, CryptoResult};

pub const KEY_LENGTH: usize = 32;

#[derive(ZeroizeOnDrop)]
pub struct SecretKey([u8; KEY_LENGTH]);

impl SecretKey {
    pub fn generate() -> Self {
        Self(<[u8; KEY_LENGTH]>::generate())
    }

    pub fn from_bytes(bytes: &[u8]) -> CryptoResult<Self> {
        let key_bytes: [u8; KEY_LENGTH] = bytes
            .try_into()
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        Ok(Self(key_bytes))
    }

    pub fn expose_bytes(&self) -> &[u8; KEY_LENGTH] {
        &self.0
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SecretKey(redacted)")
    }
}
