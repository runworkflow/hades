mod error;
mod lifecycle;
mod password;
mod recovery_key;
mod unlocked_vault;

pub use error::{VaultError, VaultResult};
pub use lifecycle::{NewVault, create_vault, unlock_with_password, unlock_with_recovery_key};
pub use password::MINIMUM_PASSWORD_LENGTH;
pub use recovery_key::RecoveryKey;
pub use unlocked_vault::UnlockedVault;
