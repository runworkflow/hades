use zeroize::Zeroizing;

use crate::error::CryptoError;
use crate::items::count_items_in_folder;
use crate::records::seal_record;
use crate::store::{Envelope, EnvelopeKind, Store};
use crate::vault::{UnlockedVault, VaultError, VaultResult};

use super::folder::{Folder, FolderData, validate_folder_name, validate_icon_name};
use super::folder_service::read_folder;

const DELETED_PAYLOAD: &[u8] = b"{}";

pub fn update_folder<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    id: &str,
    name: &str,
    icon: &str,
    now_ms: i64,
) -> VaultResult<Folder> {
    let name = validate_folder_name(name)?;
    let icon = validate_icon_name(icon)?;
    let envelope = find_folder_envelope(store, id)?;
    let current = read_folder(vault, &envelope)?;

    let data = FolderData {
        name: name.clone(),
        icon: icon.clone(),
        created_at: current.created_at,
    };
    let plaintext =
        Zeroizing::new(serde_json::to_vec(&data).map_err(|_| CryptoError::EncryptionFailed)?);
    let version = envelope.version.saturating_add(1);
    let sealed = seal_record(
        vault.vault_key(),
        EnvelopeKind::Folder,
        id,
        version,
        &plaintext,
    )?;

    store.put_envelope(&Envelope {
        id: id.to_string(),
        kind: EnvelopeKind::Folder,
        version,
        is_deleted: false,
        modified_at: now_ms,
        wrapped_key: sealed.wrapped_key,
        payload: sealed.payload,
    })?;

    Ok(Folder {
        id: id.to_string(),
        name,
        icon,
        created_at: current.created_at,
    })
}

pub fn delete_folder<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    id: &str,
    now_ms: i64,
) -> VaultResult<()> {
    let envelope = find_folder_envelope(store, id)?;
    let item_count = count_items_in_folder(store, vault, id)?;
    if item_count > 0 {
        return Err(VaultError::FolderNotEmpty { item_count });
    }

    let version = envelope.version.saturating_add(1);
    let sealed = seal_record(
        vault.vault_key(),
        EnvelopeKind::Folder,
        id,
        version,
        DELETED_PAYLOAD,
    )?;
    store.put_envelope(&Envelope {
        id: id.to_string(),
        kind: EnvelopeKind::Folder,
        version,
        is_deleted: true,
        modified_at: now_ms,
        wrapped_key: sealed.wrapped_key,
        payload: sealed.payload,
    })?;
    Ok(())
}

fn find_folder_envelope<S: Store>(store: &S, id: &str) -> VaultResult<Envelope> {
    match store.get_envelope(id)? {
        Some(envelope) if envelope.kind == EnvelopeKind::Folder && !envelope.is_deleted => {
            Ok(envelope)
        }
        _ => Err(VaultError::FolderNotFound),
    }
}
