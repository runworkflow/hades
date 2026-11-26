use crate::sealed::NONCE_LENGTH;

use super::envelope::Envelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncBaseline {
    pub version: u64,
    pub payload_nonce: [u8; NONCE_LENGTH],
}

impl SyncBaseline {
    pub fn of(envelope: &Envelope) -> Self {
        Self {
            version: envelope.version,
            payload_nonce: envelope.payload.nonce,
        }
    }
}
