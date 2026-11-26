use zeroize::Zeroizing;

use crate::error::CryptoError;
use crate::records::{conflict_copy_name, new_record_id};
use crate::store::Envelope;
use crate::vault::{UnlockedVault, VaultResult};

use super::item::MAX_ITEM_NAME_LENGTH;
use super::item_changes::seal_item_envelope;
use super::item_service::read_data;

const FIRST_VERSION: u64 = 1;

pub(crate) fn make_item_conflict_copy(
    vault: &UnlockedVault,
    loser: &Envelope,
    now_ms: i64,
) -> VaultResult<Envelope> {
    let mut data = read_data(vault, loser)?;
    data.name = conflict_copy_name(&data.name, MAX_ITEM_NAME_LENGTH);
    let plaintext =
        Zeroizing::new(serde_json::to_vec(&data).map_err(|_| CryptoError::EncryptionFailed)?);
    seal_item_envelope(
        vault,
        &new_record_id(),
        FIRST_VERSION,
        false,
        now_ms,
        &plaintext,
    )
}
