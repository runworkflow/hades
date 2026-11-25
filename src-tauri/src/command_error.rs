use hades_core::{StoreError, VaultError};
use serde::Serialize;

const INTERNAL_MESSAGE: &str = "Something went wrong. Please try again.";

#[derive(Debug, Serialize)]
pub struct CommandError {
    code: &'static str,
    message: String,
}

impl CommandError {
    fn new(code: &'static str, message: String) -> Self {
        Self { code, message }
    }

    pub fn internal() -> Self {
        Self::new("internal", INTERNAL_MESSAGE.to_string())
    }
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
            VaultError::Crypto(_) | VaultError::Store(_) => Self::internal(),
        }
    }
}

impl From<StoreError> for CommandError {
    fn from(_: StoreError) -> Self {
        Self::internal()
    }
}
