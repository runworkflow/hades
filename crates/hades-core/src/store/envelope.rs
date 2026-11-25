use crate::key_wrapping::WrappedKey;
use crate::sealed::Sealed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeKind {
    Folder,
    Item,
}

impl EnvelopeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Folder => "folder",
            Self::Item => "item",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "folder" => Some(Self::Folder),
            "item" => Some(Self::Item),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub id: String,
    pub kind: EnvelopeKind,
    pub version: u64,
    pub is_deleted: bool,
    pub modified_at: i64,
    pub wrapped_key: WrappedKey,
    pub payload: Sealed,
}
