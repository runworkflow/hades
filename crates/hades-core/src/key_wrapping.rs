use crate::error::CryptoResult;
use crate::sealed::{Sealed, open, seal};
use crate::secret_key::SecretKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrappedKey(pub Sealed);

pub fn wrap_key(
    wrapping_key: &SecretKey,
    key_to_wrap: &SecretKey,
    context: &[u8],
) -> CryptoResult<WrappedKey> {
    seal(wrapping_key, key_to_wrap.expose_bytes(), context).map(WrappedKey)
}

pub fn unwrap_key(
    wrapping_key: &SecretKey,
    wrapped: &WrappedKey,
    context: &[u8],
) -> CryptoResult<SecretKey> {
    let key_bytes = open(wrapping_key, &wrapped.0, context)?;
    SecretKey::from_bytes(&key_bytes)
}
