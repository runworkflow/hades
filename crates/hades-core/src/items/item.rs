use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

use super::item_kind::ItemKind;

pub const MAX_ITEM_NAME_LENGTH: usize = 80;
pub const MAX_FIELD_LENGTH: usize = 32_768;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemSummary {
    pub id: String,
    pub name: String,
    pub kind: ItemKind,
    pub folder_id: String,
    pub modified_at: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemField {
    pub key: String,
    pub is_secret: bool,
    pub has_value: bool,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDetail {
    pub summary: ItemSummary,
    pub fields: Vec<ItemField>,
}

#[derive(Debug, Clone)]
pub struct NewItem {
    pub folder_id: String,
    pub kind: ItemKind,
    pub name: String,
    pub fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ItemUpdate {
    pub folder_id: String,
    pub name: String,
    pub fields: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct FileData {
    pub blob_id: String,
    pub blob_key: Zeroizing<String>,
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ItemData {
    pub name: String,
    pub kind: ItemKind,
    pub folder_id: String,
    pub fields: BTreeMap<String, Zeroizing<String>>,
    pub created_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<FileData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ItemSummaryData {
    pub name: String,
    pub kind: ItemKind,
    pub folder_id: String,
}
