use zeroize::Zeroizing;

use crate::error::CryptoError;
use crate::records::{clean_name, seal_record};
use crate::store::{Envelope, EnvelopeKind, Store};
use crate::vault::{UnlockedVault, VaultResult};

use super::item::{ItemSummary, ItemUpdate, MAX_ITEM_NAME_LENGTH};
use super::item_service::{check_fields, check_folder_exists, find_item_envelope, read_data};

const DELETED_PAYLOAD: &[u8] = b"{}";

pub(super) fn seal_item_envelope(
    vault: &UnlockedVault,
    id: &str,
    version: u64,
    is_deleted: bool,
    modified_at: i64,
    plaintext: &[u8],
) -> VaultResult<Envelope> {
    let sealed = seal_record(
        vault.vault_key(),
        EnvelopeKind::Item,
        id,
        version,
        plaintext,
    )?;
    Ok(Envelope {
        id: id.to_string(),
        kind: EnvelopeKind::Item,
        version,
        is_deleted,
        modified_at,
        wrapped_key: sealed.wrapped_key,
        payload: sealed.payload,
    })
}

pub(super) fn tombstone_envelope(
    vault: &UnlockedVault,
    current: &Envelope,
    now_ms: i64,
) -> VaultResult<Envelope> {
    seal_item_envelope(
        vault,
        &current.id,
        current.version.saturating_add(1),
        true,
        now_ms,
        DELETED_PAYLOAD,
    )
}

pub fn update_item<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    id: &str,
    update: ItemUpdate,
    now_ms: i64,
) -> VaultResult<ItemSummary> {
    let envelope = find_item_envelope(store, id)?;
    let mut data = read_data(vault, &envelope)?;

    let name = clean_name(&update.name, MAX_ITEM_NAME_LENGTH)?;
    check_folder_exists(store, &update.folder_id)?;
    check_fields(data.kind, &update.fields)?;

    for (key, value) in update.fields {
        data.fields.insert(key, Zeroizing::new(value));
    }
    data.name = name.clone();
    data.folder_id = update.folder_id.clone();

    let plaintext =
        Zeroizing::new(serde_json::to_vec(&data).map_err(|_| CryptoError::EncryptionFailed)?);
    let updated = seal_item_envelope(
        vault,
        id,
        envelope.version.saturating_add(1),
        false,
        now_ms,
        &plaintext,
    )?;
    store.put_envelope(&updated)?;

    Ok(ItemSummary {
        id: id.to_string(),
        name,
        kind: data.kind,
        folder_id: update.folder_id,
        modified_at: now_ms,
    })
}

pub fn delete_item<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    id: &str,
    now_ms: i64,
) -> VaultResult<()> {
    let envelope = find_item_envelope(store, id)?;
    store.put_envelope(&tombstone_envelope(vault, &envelope, now_ms)?)?;
    Ok(())
}
