use chacha20poly1305::aead::Generate;
use data_encoding::HEXLOWER;

const RECORD_ID_BYTES: usize = 16;

pub fn new_record_id() -> String {
    HEXLOWER.encode(&<[u8; RECORD_ID_BYTES]>::generate())
}

pub fn is_valid_record_id(id: &str) -> bool {
    id.len() == RECORD_ID_BYTES * 2
        && id
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}
