use rusqlite::Row;

use crate::key_derivation::{KdfParams, SALT_LENGTH, Salt};
use crate::key_wrapping::WrappedKey;
use crate::sealed::{NONCE_LENGTH, Sealed};

use super::error::StoreError;
use super::vault_settings::VaultSettings;

pub const SELECT_SETTINGS_COLUMNS: &str =
    "created_at, kdf_salt, kdf_memory_kib, kdf_iterations, kdf_lanes,
     password_key_nonce, password_key_ciphertext, recovery_key_nonce, recovery_key_ciphertext";

pub struct StoredSettingsRow {
    created_at: i64,
    kdf_salt: Vec<u8>,
    kdf_memory_kib: u32,
    kdf_iterations: u32,
    kdf_lanes: u32,
    password_key_nonce: Vec<u8>,
    password_key_ciphertext: Vec<u8>,
    recovery_key_nonce: Vec<u8>,
    recovery_key_ciphertext: Vec<u8>,
}

impl StoredSettingsRow {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            created_at: row.get(0)?,
            kdf_salt: row.get(1)?,
            kdf_memory_kib: row.get(2)?,
            kdf_iterations: row.get(3)?,
            kdf_lanes: row.get(4)?,
            password_key_nonce: row.get(5)?,
            password_key_ciphertext: row.get(6)?,
            recovery_key_nonce: row.get(7)?,
            recovery_key_ciphertext: row.get(8)?,
        })
    }
}

fn read_nonce(bytes: Vec<u8>) -> Result<[u8; NONCE_LENGTH], StoreError> {
    bytes.try_into().map_err(|_| StoreError::CorruptRecord)
}

impl TryFrom<StoredSettingsRow> for VaultSettings {
    type Error = StoreError;

    fn try_from(row: StoredSettingsRow) -> Result<Self, Self::Error> {
        let salt_bytes: [u8; SALT_LENGTH] = row
            .kdf_salt
            .try_into()
            .map_err(|_| StoreError::CorruptRecord)?;

        Ok(VaultSettings {
            created_at: row.created_at,
            kdf_salt: Salt::from_bytes(salt_bytes),
            kdf_params: KdfParams {
                memory_kib: row.kdf_memory_kib,
                iterations: row.kdf_iterations,
                lanes: row.kdf_lanes,
            },
            password_wrapped_key: WrappedKey(Sealed {
                nonce: read_nonce(row.password_key_nonce)?,
                ciphertext: row.password_key_ciphertext,
            }),
            recovery_wrapped_key: WrappedKey(Sealed {
                nonce: read_nonce(row.recovery_key_nonce)?,
                ciphertext: row.recovery_key_ciphertext,
            }),
        })
    }
}
