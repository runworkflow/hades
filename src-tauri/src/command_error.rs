use hades_core::{BiometricError, BlobError, StoreError, VaultError};
use serde::Serialize;

const INTERNAL_MESSAGE: &str = "Something went wrong. Please try again.";

#[derive(Debug, Serialize)]
pub struct CommandError {
    code: &'static str,
    message: String,
}

impl CommandError {
    pub fn new(code: &'static str, message: String) -> Self {
        Self { code, message }
    }

    pub fn internal() -> Self {
        Self::new("internal", INTERNAL_MESSAGE.to_string())
    }
}

fn biometric_error(error: BiometricError) -> CommandError {
    match error {
        BiometricError::Unavailable => CommandError::new(
            "biometrics_unavailable",
            "Touch ID is not available on this Mac.".to_string(),
        ),
        BiometricError::Cancelled => CommandError::new(
            "biometrics_cancelled",
            "Touch ID was cancelled.".to_string(),
        ),
        BiometricError::KeyChanged => CommandError::new(
            "biometrics_changed",
            "Touch ID no longer matches this vault. Unlock with your password, then turn Touch ID on again."
                .to_string(),
        ),
        BiometricError::Failed => CommandError::new(
            "biometrics_failed",
            "Touch ID did not work. Use your password.".to_string(),
        ),
    }
}

fn file_too_large_message(maximum_bytes: u64) -> String {
    let megabytes = maximum_bytes / (1024 * 1024);
    format!("This file is larger than {megabytes} MiB and cannot be added.")
}

fn folder_not_empty_message(item_count: usize) -> String {
    let noun = if item_count == 1 { "item" } else { "items" };
    format!("Move or delete the {item_count} {noun} in this folder first.")
}

impl From<VaultError> for CommandError {
    fn from(error: VaultError) -> Self {
        let message = error.to_string();
        match error {
            VaultError::WeakPassword { .. } => Self::new("weak_password", message),
            VaultError::NotInitialized => Self::new("vault_not_created", message),
            VaultError::AlreadyInitialized => Self::new("vault_exists", message),
            VaultError::IncorrectPassword => Self::new("incorrect_password", message),
            VaultError::InvalidRecoveryKey => Self::new("invalid_recovery_key", message),
            VaultError::IncorrectRecoveryKey => Self::new("incorrect_recovery_key", message),
            VaultError::VaultLocked => Self::new("vault_locked", message),
            VaultError::InvalidName { .. } => Self::new("invalid_name", message),
            VaultError::InvalidIcon => Self::new("invalid_icon", message),
            VaultError::FolderNotFound => Self::new("folder_not_found", message),
            VaultError::FolderNotEmpty { item_count } => {
                Self::new("folder_not_empty", folder_not_empty_message(item_count))
            }
            VaultError::ItemNotFound => Self::new("item_not_found", message),
            VaultError::InvalidField => Self::new("invalid_field", message),
            VaultError::InvalidItemKind => Self::new("invalid_item_kind", message),
            VaultError::Blob(BlobError::TooLarge { maximum_bytes }) => {
                Self::new("file_too_large", file_too_large_message(maximum_bytes))
            }
            VaultError::BiometricsNotEnabled => Self::new("biometrics_not_enabled", message),
            VaultError::Biometric(error) => biometric_error(error),
            VaultError::Blob(BlobError::Corrupt) => Self::new("file_damaged", message),
            VaultError::Blob(_) | VaultError::Crypto(_) | VaultError::Store(_) => Self::internal(),
        }
    }
}

impl From<StoreError> for CommandError {
    fn from(_: StoreError) -> Self {
        Self::internal()
    }
}
