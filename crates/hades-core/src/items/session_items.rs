use zeroize::Zeroizing;

use crate::store::Store;
use crate::vault::{VaultError, VaultResult, VaultSession};

use super::item::{ItemDetail, ItemSummary, NewItem};
use super::item_service::{create_item, get_item, list_items, reveal_item_field};

impl<S: Store> VaultSession<S> {
    pub fn create_item(&self, new_item: NewItem, now_ms: i64) -> VaultResult<ItemSummary> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        create_item(self.store(), vault, new_item, now_ms)
    }

    pub fn list_items(&self) -> VaultResult<Vec<ItemSummary>> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        list_items(self.store(), vault)
    }

    pub fn get_item(&self, id: &str) -> VaultResult<ItemDetail> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        get_item(self.store(), vault, id)
    }

    pub fn reveal_item_field(&self, id: &str, key: &str) -> VaultResult<Zeroizing<String>> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        reveal_item_field(self.store(), vault, id, key)
    }
}
