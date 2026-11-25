use std::collections::BTreeSet;

use zeroize::Zeroizing;

use crate::error::CryptoError;
use crate::store::Store;
use crate::vault::{UnlockedVault, VaultResult};

use super::item_changes::{seal_item_envelope, tombstone_envelope};
use super::item_service::{check_folder_exists, find_item_envelope, read_data};

fn unique_ids(ids: &[String]) -> BTreeSet<&str> {
    ids.iter().map(String::as_str).collect()
}

pub fn delete_items<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    ids: &[String],
    now_ms: i64,
) -> VaultResult<usize> {
    let tombstones = unique_ids(ids)
        .into_iter()
        .map(|id| {
            let envelope = find_item_envelope(store, id)?;
            tombstone_envelope(vault, &envelope, now_ms)
        })
        .collect::<VaultResult<Vec<_>>>()?;

    store.put_envelopes(&tombstones)?;
    Ok(tombstones.len())
}

pub fn move_items<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    ids: &[String],
    folder_id: &str,
    now_ms: i64,
) -> VaultResult<usize> {
    check_folder_exists(store, folder_id)?;

    let moved = unique_ids(ids)
        .into_iter()
        .map(|id| {
            let envelope = find_item_envelope(store, id)?;
            let mut data = read_data(vault, &envelope)?;
            data.folder_id = folder_id.to_string();
            let plaintext = Zeroizing::new(
                serde_json::to_vec(&data).map_err(|_| CryptoError::EncryptionFailed)?,
            );
            seal_item_envelope(
                vault,
                id,
                envelope.version.saturating_add(1),
                false,
                now_ms,
                &plaintext,
            )
        })
        .collect::<VaultResult<Vec<_>>>()?;

    store.put_envelopes(&moved)?;
    Ok(moved.len())
}
