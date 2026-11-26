use std::io::{Read, Write};

use zeroize::Zeroizing;

use crate::blobs::FileBlobStore;
use crate::store::Store;
use crate::vault::{VaultError, VaultResult, VaultSession};

use super::file_items::{export_file_item, import_file_item, remove_orphan_blobs};
use super::item::{ItemDetail, ItemSummary, ItemUpdate, NewItem};
use super::item_bulk::{delete_items, move_items};
use super::item_changes::{delete_item, update_item};
use super::item_service::{create_item, get_item, list_items, read_item_field, reveal_item_field};

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

    pub fn update_item(
        &self,
        id: &str,
        update: ItemUpdate,
        now_ms: i64,
    ) -> VaultResult<ItemSummary> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        update_item(self.store(), vault, id, update, now_ms)
    }

    pub fn delete_item(&self, id: &str, now_ms: i64) -> VaultResult<()> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        delete_item(self.store(), vault, id, now_ms)
    }

    pub fn delete_items(&self, ids: &[String], now_ms: i64) -> VaultResult<usize> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        delete_items(self.store(), vault, ids, now_ms)
    }

    pub fn move_items(&self, ids: &[String], folder_id: &str, now_ms: i64) -> VaultResult<usize> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        move_items(self.store(), vault, ids, folder_id, now_ms)
    }

    pub fn import_file_item<R: Read>(
        &self,
        blob_store: &FileBlobStore,
        folder_id: &str,
        file_name: &str,
        reader: &mut R,
        now_ms: i64,
    ) -> VaultResult<ItemSummary> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        import_file_item(
            self.store(),
            vault,
            blob_store,
            folder_id,
            file_name,
            reader,
            now_ms,
        )
    }

    pub fn export_file_item<W: Write>(
        &self,
        blob_store: &FileBlobStore,
        id: &str,
        writer: &mut W,
    ) -> VaultResult<u64> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        export_file_item(self.store(), vault, blob_store, id, writer)
    }

    pub fn remove_orphan_blobs(&self, blob_store: &FileBlobStore) -> VaultResult<usize> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        remove_orphan_blobs(self.store(), vault, blob_store)
    }

    pub fn read_item_field(&self, id: &str, key: &str) -> VaultResult<Zeroizing<String>> {
        let vault = self.unlocked_vault().ok_or(VaultError::VaultLocked)?;
        read_item_field(self.store(), vault, id, key)
    }
}
