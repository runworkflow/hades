use chacha20poly1305::aead::{Aead, Generate, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use zeroize::Zeroizing;

use crate::error::{CryptoError, CryptoResult};
use crate::secret_key::SecretKey;

pub const NONCE_LENGTH: usize = 24;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sealed {
    pub nonce: [u8; NONCE_LENGTH],
    pub ciphertext: Vec<u8>,
}

fn build_cipher(key: &SecretKey) -> CryptoResult<XChaCha20Poly1305> {
    XChaCha20Poly1305::new_from_slice(key.expose_bytes()).map_err(|_| CryptoError::InvalidKeyLength)
}

pub fn seal(key: &SecretKey, plaintext: &[u8], associated_data: &[u8]) -> CryptoResult<Sealed> {
    let cipher = build_cipher(key)?;
    let nonce = XNonce::generate();
    let ciphertext = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: plaintext,
                aad: associated_data,
            },
        )
        .map_err(|_| CryptoError::EncryptionFailed)?;
    Ok(Sealed {
        nonce: nonce.into(),
        ciphertext,
    })
}

pub fn open(
    key: &SecretKey,
    sealed: &Sealed,
    associated_data: &[u8],
) -> CryptoResult<Zeroizing<Vec<u8>>> {
    let cipher = build_cipher(key)?;
    let nonce = XNonce::from(sealed.nonce);
    cipher
        .decrypt(
            &nonce,
            Payload {
                msg: &sealed.ciphertext,
                aad: associated_data,
            },
        )
        .map(Zeroizing::new)
        .map_err(|_| CryptoError::DecryptionFailed)
}
