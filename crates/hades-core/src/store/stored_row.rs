use rusqlite::Row;

use crate::key_wrapping::WrappedKey;
use crate::sealed::{NONCE_LENGTH, Sealed};

use super::envelope::{Envelope, EnvelopeKind};
use super::error::StoreError;

pub const SELECT_COLUMNS: &str =
    "id, kind, version, is_deleted, modified_at, key_nonce, key_ciphertext, nonce, ciphertext";

pub struct StoredRow {
    id: String,
    kind: String,
    version: i64,
    is_deleted: bool,
    modified_at: i64,
    key_nonce: Vec<u8>,
    key_ciphertext: Vec<u8>,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

impl StoredRow {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            kind: row.get(1)?,
            version: row.get(2)?,
            is_deleted: row.get(3)?,
            modified_at: row.get(4)?,
            key_nonce: row.get(5)?,
            key_ciphertext: row.get(6)?,
            nonce: row.get(7)?,
            ciphertext: row.get(8)?,
        })
    }
}

fn read_nonce(bytes: Vec<u8>) -> Result<[u8; NONCE_LENGTH], StoreError> {
    bytes.try_into().map_err(|_| StoreError::CorruptRecord)
}

impl TryFrom<StoredRow> for Envelope {
    type Error = StoreError;

    fn try_from(row: StoredRow) -> Result<Self, Self::Error> {
        Ok(Envelope {
            id: row.id,
            kind: EnvelopeKind::parse(&row.kind).ok_or(StoreError::CorruptRecord)?,
            version: u64::try_from(row.version).map_err(|_| StoreError::CorruptRecord)?,
            is_deleted: row.is_deleted,
            modified_at: row.modified_at,
            wrapped_key: WrappedKey(Sealed {
                nonce: read_nonce(row.key_nonce)?,
                ciphertext: row.key_ciphertext,
            }),
            payload: Sealed {
                nonce: read_nonce(row.nonce)?,
                ciphertext: row.ciphertext,
            },
        })
    }
}
