use crate::store::Store;
use crate::vault::{VaultError, VaultResult, VaultSession};

use super::folder::Folder;
use super::folder_changes::{delete_folder, update_folder};
use super::folder_service::{create_folder, list_folders};

impl<S: Store> VaultSession<S> {
    pub fn create_folder(&self, name: &str, icon: &str, now_ms: i64) -> VaultResult<Folder> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        create_folder(self.store(), vault, name, icon, now_ms)
    }

    pub fn list_folders(&self) -> VaultResult<Vec<Folder>> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        list_folders(self.store(), vault)
    }

    pub fn update_folder(
        &self,
        id: &str,
        name: &str,
        icon: &str,
        now_ms: i64,
    ) -> VaultResult<Folder> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        update_folder(self.store(), vault, id, name, icon, now_ms)
    }

    pub fn delete_folder(&self, id: &str, now_ms: i64) -> VaultResult<()> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        delete_folder(self.store(), vault, id, now_ms)
    }
}
