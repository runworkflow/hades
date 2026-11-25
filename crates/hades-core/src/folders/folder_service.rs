use zeroize::Zeroizing;

use crate::error::CryptoError;
use crate::records::{new_record_id, open_record, seal_record};
use crate::store::{Envelope, EnvelopeKind, Store, StoreError};
use crate::vault::{UnlockedVault, VaultError, VaultResult};

use super::folder::{Folder, FolderData, validate_folder_name, validate_icon_name};

const FIRST_VERSION: u64 = 1;

pub fn create_folder<S: Store>(
    store: &S,
    vault: &UnlockedVault,
    name: &str,
    icon: &str,
    now_ms: i64,
) -> VaultResult<Folder> {
    let name = validate_folder_name(name)?;
    let icon = validate_icon_name(icon)?;
    let id = new_record_id();

    let data = FolderData {
        name: name.clone(),
        icon: icon.clone(),
        created_at: now_ms,
    };
    let plaintext =
        Zeroizing::new(serde_json::to_vec(&data).map_err(|_| CryptoError::EncryptionFailed)?);
    let sealed = seal_record(
        vault.vault_key(),
        EnvelopeKind::Folder,
        &id,
        FIRST_VERSION,
        &plaintext,
    )?;

    store.put_envelope(&Envelope {
        id: id.clone(),
        kind: EnvelopeKind::Folder,
        version: FIRST_VERSION,
        is_deleted: false,
        modified_at: now_ms,
        wrapped_key: sealed.wrapped_key,
        payload: sealed.payload,
    })?;

    Ok(Folder {
        id,
        name,
        icon,
        created_at: now_ms,
    })
}

pub fn list_folders<S: Store>(store: &S, vault: &UnlockedVault) -> VaultResult<Vec<Folder>> {
    let mut folders = store
        .list_envelopes(EnvelopeKind::Folder)?
        .iter()
        .map(|envelope| read_folder(vault, envelope))
        .collect::<VaultResult<Vec<_>>>()?;
    folders.sort_by(|first, second| {
        first
            .created_at
            .cmp(&second.created_at)
            .then_with(|| first.id.cmp(&second.id))
    });
    Ok(folders)
}

fn read_folder(vault: &UnlockedVault, envelope: &Envelope) -> VaultResult<Folder> {
    let plaintext = open_record(vault.vault_key(), envelope)?;
    let data: FolderData = serde_json::from_slice(&plaintext)
        .map_err(|_| VaultError::Store(StoreError::CorruptRecord))?;
    Ok(Folder {
        id: envelope.id.clone(),
        name: data.name,
        icon: data.icon,
        created_at: data.created_at,
    })
}
