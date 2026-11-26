use zeroize::Zeroizing;

use crate::error::CryptoError;
use crate::records::{conflict_copy_name, new_record_id, open_record, seal_record};
use crate::store::{Envelope, EnvelopeKind, StoreError};
use crate::vault::{UnlockedVault, VaultError, VaultResult};

use super::folder::{FolderData, MAX_FOLDER_NAME_LENGTH};

const FIRST_VERSION: u64 = 1;

pub(crate) fn make_folder_conflict_copy(
    vault: &UnlockedVault,
    loser: &Envelope,
    now_ms: i64,
) -> VaultResult<Envelope> {
    let plaintext = open_record(vault.vault_key(), loser)?;
    let mut data: FolderData = serde_json::from_slice(&plaintext)
        .map_err(|_| VaultError::Store(StoreError::CorruptRecord))?;
    data.name = conflict_copy_name(&data.name, MAX_FOLDER_NAME_LENGTH);

    let id = new_record_id();
    let copy_plaintext =
        Zeroizing::new(serde_json::to_vec(&data).map_err(|_| CryptoError::EncryptionFailed)?);
    let sealed = seal_record(
        vault.vault_key(),
        EnvelopeKind::Folder,
        &id,
        FIRST_VERSION,
        &copy_plaintext,
    )?;
    Ok(Envelope {
        id,
        kind: EnvelopeKind::Folder,
        version: FIRST_VERSION,
        is_deleted: false,
        modified_at: now_ms,
        wrapped_key: sealed.wrapped_key,
        payload: sealed.payload,
    })
}
