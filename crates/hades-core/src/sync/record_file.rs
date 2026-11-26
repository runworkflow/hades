use data_encoding::BASE64;
use serde::{Deserialize, Serialize};

use crate::key_wrapping::WrappedKey;
use crate::sealed::{NONCE_LENGTH, Sealed};
use crate::store::{Envelope, EnvelopeKind};

use super::sync_error::{SyncError, SyncResult};

pub const FILE_FORMAT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SealedFile {
    nonce: String,
    ciphertext: String,
}

impl SealedFile {
    pub(super) fn from_sealed(sealed: &Sealed) -> Self {
        Self {
            nonce: BASE64.encode(&sealed.nonce),
            ciphertext: BASE64.encode(&sealed.ciphertext),
        }
    }

    pub(super) fn into_sealed(self) -> SyncResult<Sealed> {
        let nonce: [u8; NONCE_LENGTH] = BASE64
            .decode(self.nonce.as_bytes())
            .map_err(|_| SyncError::DamagedFile)?
            .try_into()
            .map_err(|_| SyncError::DamagedFile)?;
        let ciphertext = BASE64
            .decode(self.ciphertext.as_bytes())
            .map_err(|_| SyncError::DamagedFile)?;
        Ok(Sealed { nonce, ciphertext })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct EnvelopeFile {
    format: u32,
    id: String,
    kind: String,
    version: u64,
    is_deleted: bool,
    modified_at: i64,
    wrapped_key: SealedFile,
    payload: SealedFile,
}

pub fn encode_envelope(envelope: &Envelope) -> SyncResult<Vec<u8>> {
    let file = EnvelopeFile {
        format: FILE_FORMAT_VERSION,
        id: envelope.id.clone(),
        kind: envelope.kind.as_str().to_string(),
        version: envelope.version,
        is_deleted: envelope.is_deleted,
        modified_at: envelope.modified_at,
        wrapped_key: SealedFile::from_sealed(&envelope.wrapped_key.0),
        payload: SealedFile::from_sealed(&envelope.payload),
    };
    serde_json::to_vec(&file).map_err(|_| SyncError::DamagedFile)
}

pub fn decode_envelope(bytes: &[u8]) -> SyncResult<Envelope> {
    let file: EnvelopeFile = serde_json::from_slice(bytes).map_err(|_| SyncError::DamagedFile)?;
    if file.format != FILE_FORMAT_VERSION {
        return Err(SyncError::UnsupportedFormat);
    }
    Ok(Envelope {
        id: file.id,
        kind: EnvelopeKind::parse(&file.kind).ok_or(SyncError::DamagedFile)?,
        version: file.version,
        is_deleted: file.is_deleted,
        modified_at: file.modified_at,
        wrapped_key: WrappedKey(file.wrapped_key.into_sealed()?),
        payload: file.payload.into_sealed()?,
    })
}
