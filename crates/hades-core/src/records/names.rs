use crate::vault::{VaultError, VaultResult};

pub fn clean_name(name: &str, maximum_length: usize) -> VaultResult<String> {
    let trimmed = name.trim();
    let length = trimmed.chars().count();
    if length == 0 || length > maximum_length || trimmed.chars().any(char::is_control) {
        return Err(VaultError::InvalidName { maximum_length });
    }
    Ok(trimmed.to_string())
}
