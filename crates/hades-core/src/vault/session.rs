use crate::store::{Store, StoreResult};

use super::credentials::{change_password, regenerate_recovery_key};
use super::error::{VaultError, VaultResult};
use super::lifecycle::{create_vault, unlock_with_password, unlock_with_recovery_key};
use super::recovery_key::RecoveryKey;
use super::unlocked_vault::UnlockedVault;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultStatus {
    NotCreated,
    Locked,
    Unlocked,
}

pub struct VaultSession<S: Store> {
    store: S,
    unlocked_vault: Option<UnlockedVault>,
}

impl<S: Store> VaultSession<S> {
    pub fn new(store: S) -> Self {
        Self {
            store,
            unlocked_vault: None,
        }
    }

    pub fn status(&self) -> StoreResult<VaultStatus> {
        if self.unlocked_vault.is_some() {
            return Ok(VaultStatus::Unlocked);
        }
        let has_settings = self.store.get_vault_settings()?.is_some();
        Ok(if has_settings {
            VaultStatus::Locked
        } else {
            VaultStatus::NotCreated
        })
    }

    pub fn create(&mut self, password: &str, created_at: i64) -> VaultResult<RecoveryKey> {
        let new_vault = create_vault(&self.store, password, created_at)?;
        self.unlocked_vault = Some(new_vault.vault);
        Ok(new_vault.recovery_key)
    }

    pub fn unlock_with_password(&mut self, password: &str) -> VaultResult<()> {
        self.unlocked_vault = Some(unlock_with_password(&self.store, password)?);
        Ok(())
    }

    pub fn unlock_with_recovery_key(&mut self, recovery_key_text: &str) -> VaultResult<()> {
        self.unlocked_vault = Some(unlock_with_recovery_key(&self.store, recovery_key_text)?);
        Ok(())
    }

    pub fn lock(&mut self) {
        self.unlocked_vault = None;
    }

    pub fn change_password(&self, current_password: &str, new_password: &str) -> VaultResult<()> {
        let vault = self
            .unlocked_vault
            .as_ref()
            .ok_or(VaultError::VaultLocked)?;
        change_password(&self.store, vault, current_password, new_password)
    }

    pub fn regenerate_recovery_key(&self, current_password: &str) -> VaultResult<RecoveryKey> {
        let vault = self
            .unlocked_vault
            .as_ref()
            .ok_or(VaultError::VaultLocked)?;
        regenerate_recovery_key(&self.store, vault, current_password)
    }

    pub fn unlocked_vault(&self) -> Option<&UnlockedVault> {
        self.unlocked_vault.as_ref()
    }

    pub fn store(&self) -> &S {
        &self.store
    }
}
