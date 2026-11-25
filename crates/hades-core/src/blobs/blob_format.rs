pub const MAGIC: [u8; 4] = *b"HBLB";
pub const FORMAT_VERSION: u8 = 1;
pub const CHUNK_SIZE: usize = 1024 * 1024;
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;
pub const TAG_LENGTH: usize = 16;

pub(super) fn chunk_context(blob_id: &str, index: u64, is_last: bool) -> Vec<u8> {
    format!("hades:blob:{blob_id}:{index}:{}", u8::from(is_last)).into_bytes()
}
