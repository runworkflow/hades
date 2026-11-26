use crate::vault::{VaultError, VaultResult};

const CONFLICT_COPY_SUFFIX: &str = " (conflict copy)";

pub fn conflict_copy_name(name: &str, maximum_length: usize) -> String {
    let room = maximum_length.saturating_sub(CONFLICT_COPY_SUFFIX.chars().count());
    let shortened: String = name.chars().take(room).collect();
    format!("{}{CONFLICT_COPY_SUFFIX}", shortened.trim_end())
}

pub fn clean_name(name: &str, maximum_length: usize) -> VaultResult<String> {
    let trimmed = name.trim();
    let length = trimmed.chars().count();
    if length == 0 || length > maximum_length || trimmed.chars().any(char::is_control) {
        return Err(VaultError::InvalidName { maximum_length });
    }
    Ok(trimmed.to_string())
}
