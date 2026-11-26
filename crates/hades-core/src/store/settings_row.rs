use rusqlite::Row;

use crate::key_derivation::{KdfParams, SALT_LENGTH, Salt};
use crate::key_wrapping::WrappedKey;
use crate::sealed::{NONCE_LENGTH, Sealed};

use super::error::StoreError;
use super::vault_settings::{BiometricSettings, VaultSettings};

pub const SELECT_SETTINGS_COLUMNS: &str =
    "vault_id, settings_version, created_at, kdf_salt, kdf_memory_kib, kdf_iterations, kdf_lanes,
     password_key_nonce, password_key_ciphertext, recovery_key_nonce, recovery_key_ciphertext,
     biometric_key_blob, biometric_public_key, biometric_key_nonce, biometric_key_ciphertext";

pub struct StoredSettingsRow {
    vault_id: String,
    settings_version: i64,
    created_at: i64,
    kdf_salt: Vec<u8>,
    kdf_memory_kib: u32,
    kdf_iterations: u32,
    kdf_lanes: u32,
    password_key_nonce: Vec<u8>,
    password_key_ciphertext: Vec<u8>,
    recovery_key_nonce: Vec<u8>,
    recovery_key_ciphertext: Vec<u8>,
    biometric_key_blob: Option<Vec<u8>>,
    biometric_public_key: Option<Vec<u8>>,
    biometric_key_nonce: Option<Vec<u8>>,
    biometric_key_ciphertext: Option<Vec<u8>>,
}

impl StoredSettingsRow {
    pub fn from_row(row: &Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            vault_id: row.get(0)?,
            settings_version: row.get(1)?,
            created_at: row.get(2)?,
            kdf_salt: row.get(3)?,
            kdf_memory_kib: row.get(4)?,
            kdf_iterations: row.get(5)?,
            kdf_lanes: row.get(6)?,
            password_key_nonce: row.get(7)?,
            password_key_ciphertext: row.get(8)?,
            recovery_key_nonce: row.get(9)?,
            recovery_key_ciphertext: row.get(10)?,
            biometric_key_blob: row.get(11)?,
            biometric_public_key: row.get(12)?,
            biometric_key_nonce: row.get(13)?,
            biometric_key_ciphertext: row.get(14)?,
        })
    }
}

fn read_nonce(bytes: Vec<u8>) -> Result<[u8; NONCE_LENGTH], StoreError> {
    bytes.try_into().map_err(|_| StoreError::CorruptRecord)
}

fn read_biometric(row: &mut StoredSettingsRow) -> Result<Option<BiometricSettings>, StoreError> {
    match (
        row.biometric_key_blob.take(),
        row.biometric_public_key.take(),
        row.biometric_key_nonce.take(),
        row.biometric_key_ciphertext.take(),
    ) {
        (None, None, None, None) => Ok(None),
        (Some(key_blob), Some(ephemeral_public_key), Some(nonce), Some(ciphertext)) => {
            Ok(Some(BiometricSettings {
                key_blob,
                ephemeral_public_key,
                wrapped_key: WrappedKey(Sealed {
                    nonce: read_nonce(nonce)?,
                    ciphertext,
                }),
            }))
        }
        _ => Err(StoreError::CorruptRecord),
    }
}

impl TryFrom<StoredSettingsRow> for VaultSettings {
    type Error = StoreError;

    fn try_from(mut row: StoredSettingsRow) -> Result<Self, Self::Error> {
        let biometric = read_biometric(&mut row)?;
        let salt_bytes: [u8; SALT_LENGTH] = row
            .kdf_salt
            .try_into()
            .map_err(|_| StoreError::CorruptRecord)?;

        Ok(VaultSettings {
            vault_id: row.vault_id,
            settings_version: u64::try_from(row.settings_version)
                .map_err(|_| StoreError::CorruptRecord)?,
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
            biometric,
        })
    }
}
