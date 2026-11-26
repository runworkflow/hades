use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use chacha20poly1305::aead::Generate;
use data_encoding::HEXLOWER;

const OWNER_ONLY_FILE_MODE: u32 = 0o600;
const SUFFIX_BYTES: usize = 8;
const MAXIMUM_NAME_ATTEMPTS: usize = 8;

fn random_suffix() -> String {
    HEXLOWER.encode(&<[u8; SUFFIX_BYTES]>::generate())
}

pub fn create_exclusive_temporary_file(directory: &Path, label: &str) -> Result<(File, PathBuf)> {
    for _ in 0..MAXIMUM_NAME_ATTEMPTS {
        let path = directory.join(format!(".{label}.{}.tmp", random_suffix()));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .custom_flags(libc::O_NOFOLLOW)
            .mode(OWNER_ONLY_FILE_MODE)
            .open(&path)
        {
            Ok(file) => return Ok((file, path)),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(Error::new(
        ErrorKind::AlreadyExists,
        "no free temporary file name",
    ))
}

pub fn open_regular_file(path: &Path) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)?;
    if !file.metadata()?.is_file() {
        return Err(Error::new(ErrorKind::InvalidInput, "not a regular file"));
    }
    Ok(file)
}
