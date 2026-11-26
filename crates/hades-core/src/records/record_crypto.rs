use zeroize::Zeroizing;

use crate::error::CryptoResult;
use crate::key_wrapping::{WrappedKey, unwrap_key, wrap_key};
use crate::sealed::{Sealed, open, seal};
use crate::secret_key::SecretKey;
use crate::store::{Envelope, EnvelopeKind};

pub const DELETED_PAYLOAD: &[u8] = b"{}";

pub fn is_deleted_payload(plaintext: &[u8]) -> bool {
    plaintext == DELETED_PAYLOAD
}

pub struct SealedRecord {
    pub wrapped_key: WrappedKey,
    pub payload: Sealed,
}

fn key_context(kind: EnvelopeKind, id: &str) -> Vec<u8> {
    format!("hades:record-key:{}:{}", kind.as_str(), id).into_bytes()
}

fn payload_context(kind: EnvelopeKind, id: &str, version: u64) -> Vec<u8> {
    format!("hades:record:{}:{}:{}", kind.as_str(), id, version).into_bytes()
}

pub fn seal_record(
    vault_key: &SecretKey,
    kind: EnvelopeKind,
    id: &str,
    version: u64,
    plaintext: &[u8],
) -> CryptoResult<SealedRecord> {
    let record_key = SecretKey::generate();
    Ok(SealedRecord {
        wrapped_key: wrap_key(vault_key, &record_key, &key_context(kind, id))?,
        payload: seal(&record_key, plaintext, &payload_context(kind, id, version))?,
    })
}

pub fn open_record(vault_key: &SecretKey, envelope: &Envelope) -> CryptoResult<Zeroizing<Vec<u8>>> {
    let record_key = unwrap_key(
        vault_key,
        &envelope.wrapped_key,
        &key_context(envelope.kind, &envelope.id),
    )?;
    open(
        &record_key,
        &envelope.payload,
        &payload_context(envelope.kind, &envelope.id, envelope.version),
    )
}
