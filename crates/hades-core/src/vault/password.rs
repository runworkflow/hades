use unicode_normalization::UnicodeNormalization;
use zeroize::Zeroizing;

use super::error::{VaultError, VaultResult};

pub const MINIMUM_PASSWORD_LENGTH: usize = 12;

pub fn normalize_password(password: &str) -> Zeroizing<String> {
    Zeroizing::new(password.nfkc().collect())
}

pub fn check_password_strength(normalized_password: &str) -> VaultResult<()> {
    if normalized_password.chars().count() < MINIMUM_PASSWORD_LENGTH {
        return Err(VaultError::WeakPassword {
            minimum_length: MINIMUM_PASSWORD_LENGTH,
        });
    }
    Ok(())
}
