use chacha20poly1305::aead::Generate;
use data_encoding::HEXLOWER;

const RECORD_ID_BYTES: usize = 16;

pub fn new_record_id() -> String {
    HEXLOWER.encode(&<[u8; RECORD_ID_BYTES]>::generate())
}
