use serde::{Deserialize, Serialize};

use crate::key_derivation::{KdfParams, SALT_LENGTH, Salt};
use crate::key_wrapping::WrappedKey;
use crate::records::is_valid_record_id;
use crate::store::{BiometricSettings, VaultSettings};

use super::record_file::{FILE_FORMAT_VERSION, SealedFile};
use super::sync_error::{SyncError, SyncResult};

const MAXIMUM_MEMORY_KIB: u32 = 1_048_576;
const MAXIMUM_ITERATIONS: u32 = 64;
const MAXIMUM_LANES: u32 = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncedSettings {
    pub vault_id: String,
    pub settings_version: u64,
    pub created_at: i64,
    pub kdf_salt: Salt,
    pub kdf_params: KdfParams,
    pub password_wrapped_key: WrappedKey,
    pub recovery_wrapped_key: WrappedKey,
}

impl SyncedSettings {
    pub fn from_vault_settings(settings: &VaultSettings) -> Self {
        Self {
            vault_id: settings.vault_id.clone(),
            settings_version: settings.settings_version,
            created_at: settings.created_at,
            kdf_salt: settings.kdf_salt,
            kdf_params: settings.kdf_params,
            password_wrapped_key: settings.password_wrapped_key.clone(),
            recovery_wrapped_key: settings.recovery_wrapped_key.clone(),
        }
    }

    pub fn into_vault_settings(self, biometric: Option<BiometricSettings>) -> VaultSettings {
        VaultSettings {
            vault_id: self.vault_id,
            settings_version: self.settings_version,
            created_at: self.created_at,
            kdf_salt: self.kdf_salt,
            kdf_params: self.kdf_params,
            password_wrapped_key: self.password_wrapped_key,
            recovery_wrapped_key: self.recovery_wrapped_key,
            authentication: None,
            biometric,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SettingsFile {
    format: u32,
    vault_id: String,
    settings_version: u64,
    created_at: i64,
    kdf_salt: String,
    kdf_memory_kib: u32,
    kdf_iterations: u32,
    kdf_lanes: u32,
    password_wrapped_key: SealedFile,
    recovery_wrapped_key: SealedFile,
}

fn are_kdf_params_reasonable(params: &KdfParams) -> bool {
    (1..=MAXIMUM_MEMORY_KIB).contains(&params.memory_kib)
        && (1..=MAXIMUM_ITERATIONS).contains(&params.iterations)
        && (1..=MAXIMUM_LANES).contains(&params.lanes)
}

pub fn encode_settings(settings: &SyncedSettings) -> SyncResult<Vec<u8>> {
    let file = SettingsFile {
        format: FILE_FORMAT_VERSION,
        vault_id: settings.vault_id.clone(),
        settings_version: settings.settings_version,
        created_at: settings.created_at,
        kdf_salt: data_encoding::BASE64.encode(settings.kdf_salt.as_bytes()),
        kdf_memory_kib: settings.kdf_params.memory_kib,
        kdf_iterations: settings.kdf_params.iterations,
        kdf_lanes: settings.kdf_params.lanes,
        password_wrapped_key: SealedFile::from_sealed(&settings.password_wrapped_key.0),
        recovery_wrapped_key: SealedFile::from_sealed(&settings.recovery_wrapped_key.0),
    };
    serde_json::to_vec(&file).map_err(|_| SyncError::DamagedFile)
}

pub fn decode_settings(bytes: &[u8]) -> SyncResult<SyncedSettings> {
    let file: SettingsFile = serde_json::from_slice(bytes).map_err(|_| SyncError::DamagedFile)?;
    if file.format != FILE_FORMAT_VERSION {
        return Err(SyncError::UnsupportedFormat);
    }
    let salt_bytes: [u8; SALT_LENGTH] = data_encoding::BASE64
        .decode(file.kdf_salt.as_bytes())
        .map_err(|_| SyncError::DamagedFile)?
        .try_into()
        .map_err(|_| SyncError::DamagedFile)?;
    let kdf_params = KdfParams {
        memory_kib: file.kdf_memory_kib,
        iterations: file.kdf_iterations,
        lanes: file.kdf_lanes,
    };
    if !is_valid_record_id(&file.vault_id) || !are_kdf_params_reasonable(&kdf_params) {
        return Err(SyncError::DamagedFile);
    }
    Ok(SyncedSettings {
        vault_id: file.vault_id,
        settings_version: file.settings_version,
        created_at: file.created_at,
        kdf_salt: Salt::from_bytes(salt_bytes),
        kdf_params,
        password_wrapped_key: WrappedKey(file.password_wrapped_key.into_sealed()?),
        recovery_wrapped_key: WrappedKey(file.recovery_wrapped_key.into_sealed()?),
    })
}
