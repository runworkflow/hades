use serde::{Deserialize, Serialize};

use crate::records::clean_name;
use crate::vault::{VaultError, VaultResult};

pub const MAX_FOLDER_NAME_LENGTH: usize = 40;
pub const MAX_ICON_NAME_LENGTH: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize)]
pub(super) struct FolderData {
    pub name: String,
    pub icon: String,
    pub created_at: i64,
}

pub fn validate_folder_name(name: &str) -> VaultResult<String> {
    clean_name(name, MAX_FOLDER_NAME_LENGTH)
}

pub fn validate_icon_name(icon: &str) -> VaultResult<String> {
    let is_valid = !icon.is_empty()
        && icon.len() <= MAX_ICON_NAME_LENGTH
        && icon.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        });
    if !is_valid {
        return Err(VaultError::InvalidIcon);
    }
    Ok(icon.to_string())
}
