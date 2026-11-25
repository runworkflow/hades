mod auto_lock;
mod error;
mod lifecycle;
mod password;
mod recovery_key;
mod session;
mod unlocked_vault;

pub use auto_lock::{AutoLockMonitor, AutoLockPolicy, LockReason};
pub use error::{VaultError, VaultResult};
pub use lifecycle::{NewVault, create_vault, unlock_with_password, unlock_with_recovery_key};
pub use password::MINIMUM_PASSWORD_LENGTH;
pub use recovery_key::RecoveryKey;
pub use session::{VaultSession, VaultStatus};
pub use unlocked_vault::UnlockedVault;
