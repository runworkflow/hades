use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};

use data_encoding::HEXLOWER;
use zeroize::Zeroizing;

use crate::blobs::{BlobError, FileBlobStore};
use crate::records::{clean_name, new_record_id};
use crate::secret_key::SecretKey;
use crate::store::{EnvelopeKind, Store, StoreError};
use crate::vault::{UnlockedVault, VaultError, VaultResult};

use super::item::{FileData, ItemData, ItemSummary, MAX_ITEM_NAME_LENGTH};
use super::item_kind::ItemKind;
use super::item_service::{check_folder_exists, find_item_envelope, read_data, save_new_item};

const MAX_FILE_NAME_LENGTH: usize = 255;
const FALLBACK_FILE_NAME: &str = "file";

pub fn clean_file_name(name: &str) -> String {
    let base_name = name.rsplit(['/', '\\']).next().unwrap_or("");
    let cleaned: String = base_name
        .chars()
        .filter(|character| !character.is_control())
        .take(MAX_FILE_NAME_LENGTH)
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() || trimmed == "." || trimmed == ".." {
        FALLBACK_FILE_NAME.to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn import_file_item<S: Store, R: Read>(
    store: &S,
    vault: &UnlockedVault,
    blob_store: &FileBlobStore,
    folder_id: &str,
    file_name: &str,
    reader: &mut R,
    now_ms: i64,
) -> VaultResult<ItemSummary> {
    check_folder_exists(store, folder_id)?;
    let file_name = clean_file_name(file_name);
    let item_name = clean_name(
        &file_name
            .chars()
            .take(MAX_ITEM_NAME_LENGTH)
            .collect::<String>(),
        MAX_ITEM_NAME_LENGTH,
    )?;

    let blob_id = new_record_id();
    let blob_key = SecretKey::generate();
    let size = blob_store.write(&blob_id, &blob_key, reader)?;

    let fields: BTreeMap<String, Zeroizing<String>> = [
        ("fileName".to_string(), Zeroizing::new(file_name)),
        ("size".to_string(), Zeroizing::new(size.to_string())),
    ]
    .into();
    let data = ItemData {
        name: item_name,
        kind: ItemKind::File,
        folder_id: folder_id.to_string(),
        fields,
        created_at: now_ms,
        file: Some(FileData {
            blob_id: blob_id.clone(),
            blob_key: Zeroizing::new(HEXLOWER.encode(blob_key.expose_bytes())),
            size,
        }),
    };

    match save_new_item(store, vault, &new_record_id(), data) {
        Ok(summary) => Ok(summary),
        Err(error) => {
            let _ = blob_store.delete(&blob_id);
            Err(error)
        }
    }
}

pub fn export_file_item<S: Store, W: Write>(
    store: &S,
    vault: &UnlockedVault,
    blob_store: &FileBlobStore,
    id: &str,
    writer: &mut W,
) -> VaultResult<u64> {
    let envelope = find_item_envelope(store, id)?;
    let file = read_data(vault, &envelope)?
        .file
        .ok_or(VaultError::ItemNotFound)?;

    let key_bytes = Zeroizing::new(
        HEXLOWER
            .decode(file.blob_key.as_bytes())
            .map_err(|_| VaultError::Store(StoreError::CorruptRecord))?,
    );
    let key = SecretKey::from_bytes(&key_bytes)?;

    let size = blob_store.read(&file.blob_id, &key, writer)?;
    if size != file.size {
        return Err(VaultError::Blob(BlobError::Corrupt));
    }
    Ok(size)
}

pub fn remove_orphan_blobs<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    blob_store: &FileBlobStore,
) -> VaultResult<usize> {
    let mut referenced = BTreeSet::new();
    for envelope in store.list_envelopes(EnvelopeKind::Item)? {
        if let Some(file) = read_data(vault, &envelope)?.file {
            referenced.insert(file.blob_id);
        }
    }

    let mut removed = 0;
    for id in blob_store.list_ids()? {
        if !referenced.contains(&id) {
            blob_store.delete(&id)?;
            removed += 1;
        }
    }
    blob_store.remove_temporary_files()?;
    Ok(removed)
}
