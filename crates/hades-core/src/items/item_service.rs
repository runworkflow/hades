use std::collections::BTreeMap;

use zeroize::Zeroizing;

use crate::error::CryptoError;
use crate::records::{clean_name, new_record_id, open_record, seal_record};
use crate::store::{Envelope, EnvelopeKind, Store, StoreError};
use crate::vault::{UnlockedVault, VaultError, VaultResult};

use super::item::{
    ItemData, ItemDetail, ItemField, ItemSummary, ItemSummaryData, MAX_FIELD_LENGTH,
    MAX_ITEM_NAME_LENGTH, NewItem,
};
use super::item_kind::ItemKind;

const FIRST_VERSION: u64 = 1;

pub fn create_item<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    new_item: NewItem,
    now_ms: i64,
) -> VaultResult<ItemSummary> {
    let name = clean_name(&new_item.name, MAX_ITEM_NAME_LENGTH)?;
    check_folder_exists(store, &new_item.folder_id)?;
    let fields = clean_fields(new_item.kind, new_item.fields)?;
    let id = new_record_id();

    let data = ItemData {
        name: name.clone(),
        kind: new_item.kind,
        folder_id: new_item.folder_id.clone(),
        fields,
        created_at: now_ms,
    };
    let plaintext =
        Zeroizing::new(serde_json::to_vec(&data).map_err(|_| CryptoError::EncryptionFailed)?);
    let sealed = seal_record(
        vault.vault_key(),
        EnvelopeKind::Item,
        &id,
        FIRST_VERSION,
        &plaintext,
    )?;

    store.put_envelope(&Envelope {
        id: id.clone(),
        kind: EnvelopeKind::Item,
        version: FIRST_VERSION,
        is_deleted: false,
        modified_at: now_ms,
        wrapped_key: sealed.wrapped_key,
        payload: sealed.payload,
    })?;

    Ok(ItemSummary {
        id,
        name,
        kind: new_item.kind,
        folder_id: new_item.folder_id,
        modified_at: now_ms,
    })
}

pub fn list_items<S: Store>(store: &S, vault: &UnlockedVault) -> VaultResult<Vec<ItemSummary>> {
    let mut items = store
        .list_envelopes(EnvelopeKind::Item)?
        .iter()
        .map(|envelope| read_summary(vault, envelope))
        .collect::<VaultResult<Vec<_>>>()?;
    items.sort_by(|first, second| {
        second
            .modified_at
            .cmp(&first.modified_at)
            .then_with(|| first.id.cmp(&second.id))
    });
    Ok(items)
}

pub(crate) fn count_items_in_folder<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    folder_id: &str,
) -> VaultResult<usize> {
    let items = list_items(store, vault)?;
    Ok(items
        .iter()
        .filter(|item| item.folder_id == folder_id)
        .count())
}

pub fn get_item<S: Store>(store: &S, vault: &UnlockedVault, id: &str) -> VaultResult<ItemDetail> {
    let envelope = find_item_envelope(store, id)?;
    let data = read_data(vault, &envelope)?;

    let fields = data
        .kind
        .fields()
        .iter()
        .map(|spec| {
            let stored = data.fields.get(spec.key);
            ItemField {
                key: spec.key.to_string(),
                is_secret: spec.is_secret,
                has_value: stored.is_some_and(|value| !value.is_empty()),
                value: if spec.is_secret {
                    None
                } else {
                    Some(stored.map(|value| value.to_string()).unwrap_or_default())
                },
            }
        })
        .collect();

    Ok(ItemDetail {
        summary: ItemSummary {
            id: envelope.id,
            name: data.name,
            kind: data.kind,
            folder_id: data.folder_id,
            modified_at: envelope.modified_at,
        },
        fields,
    })
}

pub fn reveal_item_field<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    id: &str,
    key: &str,
) -> VaultResult<Zeroizing<String>> {
    read_field(store, vault, id, key, true)
}

pub fn read_item_field<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    id: &str,
    key: &str,
) -> VaultResult<Zeroizing<String>> {
    read_field(store, vault, id, key, false)
}

fn read_field<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    id: &str,
    key: &str,
    only_secret: bool,
) -> VaultResult<Zeroizing<String>> {
    let envelope = find_item_envelope(store, id)?;
    let data = read_data(vault, &envelope)?;

    let spec = data.kind.field(key).ok_or(VaultError::InvalidField)?;
    if only_secret && !spec.is_secret {
        return Err(VaultError::InvalidField);
    }
    Ok(data
        .fields
        .get(key)
        .cloned()
        .unwrap_or_else(|| Zeroizing::new(String::new())))
}

pub(super) fn check_folder_exists<S: Store>(store: &S, folder_id: &str) -> VaultResult<()> {
    match store.get_envelope(folder_id)? {
        Some(envelope) if envelope.kind == EnvelopeKind::Folder && !envelope.is_deleted => Ok(()),
        _ => Err(VaultError::FolderNotFound),
    }
}

pub(super) fn find_item_envelope<S: Store>(store: &S, id: &str) -> VaultResult<Envelope> {
    match store.get_envelope(id)? {
        Some(envelope) if envelope.kind == EnvelopeKind::Item && !envelope.is_deleted => {
            Ok(envelope)
        }
        _ => Err(VaultError::ItemNotFound),
    }
}

pub(super) fn check_fields(kind: ItemKind, fields: &BTreeMap<String, String>) -> VaultResult<()> {
    for (key, value) in fields {
        if kind.field(key).is_none() || value.chars().count() > MAX_FIELD_LENGTH {
            return Err(VaultError::InvalidField);
        }
    }
    Ok(())
}

fn clean_fields(
    kind: ItemKind,
    fields: BTreeMap<String, String>,
) -> VaultResult<BTreeMap<String, Zeroizing<String>>> {
    check_fields(kind, &fields)?;

    let mut cleaned: BTreeMap<String, Zeroizing<String>> = kind
        .fields()
        .iter()
        .map(|spec| (spec.key.to_string(), Zeroizing::new(String::new())))
        .collect();
    for (key, value) in fields {
        cleaned.insert(key, Zeroizing::new(value));
    }
    Ok(cleaned)
}

pub(super) fn read_data(vault: &UnlockedVault, envelope: &Envelope) -> VaultResult<ItemData> {
    let plaintext = open_record(vault.vault_key(), envelope)?;
    serde_json::from_slice(&plaintext).map_err(|_| VaultError::Store(StoreError::CorruptRecord))
}

fn read_summary(vault: &UnlockedVault, envelope: &Envelope) -> VaultResult<ItemSummary> {
    let plaintext = open_record(vault.vault_key(), envelope)?;
    let data: ItemSummaryData = serde_json::from_slice(&plaintext)
        .map_err(|_| VaultError::Store(StoreError::CorruptRecord))?;
    Ok(ItemSummary {
        id: envelope.id.clone(),
        name: data.name,
        kind: data.kind,
        folder_id: data.folder_id,
        modified_at: envelope.modified_at,
    })
}
