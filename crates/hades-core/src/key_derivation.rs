use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::Generate;
use zeroize::Zeroizing;

use crate::error::{CryptoError, CryptoResult};
use crate::secret_key::{KEY_LENGTH, SecretKey};

pub const SALT_LENGTH: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Salt([u8; SALT_LENGTH]);

impl Salt {
    pub fn generate() -> Self {
        Self(<[u8; SALT_LENGTH]>::generate())
    }

    pub fn from_bytes(bytes: [u8; SALT_LENGTH]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; SALT_LENGTH] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KdfParams {
    pub memory_kib: u32,
    pub iterations: u32,
    pub lanes: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        Self {
            memory_kib: 256 * 1024,
            iterations: 4,
            lanes: 4,
        }
    }
}

pub fn derive_key(password: &[u8], salt: &Salt, params: &KdfParams) -> CryptoResult<SecretKey> {
    let argon_params = Params::new(
        params.memory_kib,
        params.iterations,
        params.lanes,
        Some(KEY_LENGTH),
    )
    .map_err(|_| CryptoError::InvalidKdfParams)?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon_params);

    let mut output = Zeroizing::new([0u8; KEY_LENGTH]);
    argon
        .hash_password_into(password, salt.as_bytes(), &mut output[..])
        .map_err(|_| CryptoError::KeyDerivationFailed)?;
    SecretKey::from_bytes(&output[..])
}
