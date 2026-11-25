use std::io::{ErrorKind, Read, Write};

use zeroize::Zeroizing;

use crate::sealed::{NONCE_LENGTH, Sealed, open, seal};
use crate::secret_key::SecretKey;

use super::blob_error::{BlobError, BlobResult};
use super::blob_format::{
    CHUNK_SIZE, FORMAT_VERSION, MAGIC, MAX_FILE_SIZE, TAG_LENGTH, chunk_context,
};

const FILE_HEADER_LENGTH: usize = MAGIC.len() + 1;
const CHUNK_HEADER_LENGTH: usize = 1 + NONCE_LENGTH + 4;

fn read_chunk<R: Read>(reader: &mut R) -> BlobResult<Zeroizing<Vec<u8>>> {
    let mut buffer = Zeroizing::new(vec![0u8; CHUNK_SIZE]);
    let mut filled = 0;
    while filled < CHUNK_SIZE {
        match reader.read(&mut buffer[filled..]) {
            Ok(0) => break,
            Ok(read_count) => filled += read_count,
            Err(error) if error.kind() == ErrorKind::Interrupted => continue,
            Err(error) => return Err(error.into()),
        }
    }
    buffer.truncate(filled);
    Ok(buffer)
}

fn write_chunk<W: Write>(
    key: &SecretKey,
    blob_id: &str,
    index: u64,
    is_last: bool,
    plaintext: &[u8],
    writer: &mut W,
) -> BlobResult<()> {
    let sealed = seal(key, plaintext, &chunk_context(blob_id, index, is_last))?;
    let ciphertext_length =
        u32::try_from(sealed.ciphertext.len()).map_err(|_| BlobError::Corrupt)?;
    writer.write_all(&[u8::from(is_last)])?;
    writer.write_all(&sealed.nonce)?;
    writer.write_all(&ciphertext_length.to_le_bytes())?;
    writer.write_all(&sealed.ciphertext)?;
    Ok(())
}

pub fn encrypt_stream<R: Read, W: Write>(
    key: &SecretKey,
    blob_id: &str,
    reader: &mut R,
    writer: &mut W,
) -> BlobResult<u64> {
    writer.write_all(&MAGIC)?;
    writer.write_all(&[FORMAT_VERSION])?;

    let mut total_bytes: u64 = 0;
    let mut index: u64 = 0;
    let mut current = read_chunk(reader)?;
    loop {
        total_bytes += current.len() as u64;
        if total_bytes > MAX_FILE_SIZE {
            return Err(BlobError::TooLarge {
                maximum_bytes: MAX_FILE_SIZE,
            });
        }
        let next = if current.len() == CHUNK_SIZE {
            read_chunk(reader)?
        } else {
            Zeroizing::new(Vec::new())
        };
        let is_last = next.is_empty();
        write_chunk(key, blob_id, index, is_last, &current, writer)?;
        if is_last {
            return Ok(total_bytes);
        }
        current = next;
        index += 1;
    }
}

fn read_exact_or_corrupt<R: Read>(reader: &mut R, buffer: &mut [u8]) -> BlobResult<()> {
    reader.read_exact(buffer).map_err(|error| {
        if error.kind() == ErrorKind::UnexpectedEof {
            BlobError::Corrupt
        } else {
            BlobError::Io(error)
        }
    })
}

pub fn decrypt_stream<R: Read, W: Write>(
    key: &SecretKey,
    blob_id: &str,
    reader: &mut R,
    writer: &mut W,
) -> BlobResult<u64> {
    let mut file_header = [0u8; FILE_HEADER_LENGTH];
    read_exact_or_corrupt(reader, &mut file_header)?;
    if file_header[..MAGIC.len()] != MAGIC || file_header[MAGIC.len()] != FORMAT_VERSION {
        return Err(BlobError::Corrupt);
    }

    let mut total_bytes: u64 = 0;
    let mut index: u64 = 0;
    loop {
        let mut chunk_header = [0u8; CHUNK_HEADER_LENGTH];
        read_exact_or_corrupt(reader, &mut chunk_header)?;
        let is_last = match chunk_header[0] {
            0 => false,
            1 => true,
            _ => return Err(BlobError::Corrupt),
        };
        let nonce: [u8; NONCE_LENGTH] = chunk_header[1..1 + NONCE_LENGTH]
            .try_into()
            .map_err(|_| BlobError::Corrupt)?;
        let length_bytes: [u8; 4] = chunk_header[1 + NONCE_LENGTH..]
            .try_into()
            .map_err(|_| BlobError::Corrupt)?;
        let ciphertext_length = u32::from_le_bytes(length_bytes) as usize;
        if !(TAG_LENGTH..=CHUNK_SIZE + TAG_LENGTH).contains(&ciphertext_length) {
            return Err(BlobError::Corrupt);
        }

        let mut ciphertext = vec![0u8; ciphertext_length];
        read_exact_or_corrupt(reader, &mut ciphertext)?;
        let sealed = Sealed { nonce, ciphertext };
        let plaintext = open(key, &sealed, &chunk_context(blob_id, index, is_last))
            .map_err(|_| BlobError::Corrupt)?;

        if !is_last && plaintext.len() != CHUNK_SIZE {
            return Err(BlobError::Corrupt);
        }
        total_bytes += plaintext.len() as u64;
        if total_bytes > MAX_FILE_SIZE {
            return Err(BlobError::Corrupt);
        }
        writer.write_all(&plaintext)?;

        if is_last {
            let mut trailing = [0u8; 1];
            return match reader.read(&mut trailing)? {
                0 => Ok(total_bytes),
                _ => Err(BlobError::Corrupt),
            };
        }
        index += 1;
    }
}
