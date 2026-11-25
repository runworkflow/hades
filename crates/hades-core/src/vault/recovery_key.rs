use std::fmt;

use data_encoding::{BASE32_NOPAD, BASE32_NOPAD_NOCASE};
use zeroize::Zeroizing;

use crate::secret_key::SecretKey;

use super::error::{VaultError, VaultResult};

const CHARACTERS_PER_GROUP: usize = 4;
const DISPLAY_TEXT_CAPACITY: usize = 64;

pub struct RecoveryKey(SecretKey);

impl RecoveryKey {
    pub fn generate() -> Self {
        Self(SecretKey::generate())
    }

    pub fn parse(text: &str) -> VaultResult<Self> {
        let cleaned: Zeroizing<String> = Zeroizing::new(
            text.chars()
                .filter(|character| !character.is_whitespace() && *character != '-')
                .collect(),
        );
        let decoded = Zeroizing::new(
            BASE32_NOPAD_NOCASE
                .decode(cleaned.as_bytes())
                .map_err(|_| VaultError::InvalidRecoveryKey)?,
        );
        SecretKey::from_bytes(&decoded)
            .map(Self)
            .map_err(|_| VaultError::InvalidRecoveryKey)
    }

    pub fn to_display_text(&self) -> Zeroizing<String> {
        let encoded = Zeroizing::new(BASE32_NOPAD.encode(self.0.expose_bytes()));
        let mut grouped = Zeroizing::new(String::with_capacity(DISPLAY_TEXT_CAPACITY));
        for (index, character) in encoded.chars().enumerate() {
            if index > 0 && index % CHARACTERS_PER_GROUP == 0 {
                grouped.push('-');
            }
            grouped.push(character);
        }
        grouped
    }

    pub(super) fn as_secret_key(&self) -> &SecretKey {
        &self.0
    }
}

impl fmt::Debug for RecoveryKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RecoveryKey(redacted)")
    }
}
