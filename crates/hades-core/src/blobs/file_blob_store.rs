use std::fs::{self, DirBuilder, File, OpenOptions, Permissions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use crate::secret_key::SecretKey;

use super::blob_error::{BlobError, BlobResult};
use super::blob_stream::{decrypt_stream, encrypt_stream};

const OWNER_ONLY_DIRECTORY_MODE: u32 = 0o700;
const OWNER_ONLY_FILE_MODE: u32 = 0o600;
const BLOB_EXTENSION: &str = "blob";
const TEMPORARY_EXTENSION: &str = "tmp";
const BLOB_ID_LENGTH: usize = 32;

fn is_valid_blob_id(id: &str) -> bool {
    id.len() == BLOB_ID_LENGTH
        && id
            .chars()
            .all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

pub struct FileBlobStore {
    directory: PathBuf,
}

impl FileBlobStore {
    pub fn open(directory: &Path) -> BlobResult<Self> {
        DirBuilder::new()
            .recursive(true)
            .mode(OWNER_ONLY_DIRECTORY_MODE)
            .create(directory)?;
        fs::set_permissions(directory, Permissions::from_mode(OWNER_ONLY_DIRECTORY_MODE))?;
        Ok(Self {
            directory: directory.to_path_buf(),
        })
    }

    fn path_with_extension(&self, id: &str, extension: &str) -> BlobResult<PathBuf> {
        if !is_valid_blob_id(id) {
            return Err(BlobError::InvalidId);
        }
        Ok(self.directory.join(format!("{id}.{extension}")))
    }

    pub(crate) fn final_path(&self, id: &str) -> BlobResult<PathBuf> {
        self.path_with_extension(id, BLOB_EXTENSION)
    }

    pub fn write<R: Read>(&self, id: &str, key: &SecretKey, reader: &mut R) -> BlobResult<u64> {
        let final_path = self.path_with_extension(id, BLOB_EXTENSION)?;
        let temporary_path = self.path_with_extension(id, TEMPORARY_EXTENSION)?;
        if final_path.exists() {
            return Err(BlobError::AlreadyExists);
        }

        match write_encrypted(&temporary_path, id, key, reader) {
            Ok(size) => {
                fs::rename(&temporary_path, &final_path)?;
                Ok(size)
            }
            Err(error) => {
                let _ = fs::remove_file(&temporary_path);
                Err(error)
            }
        }
    }

    pub fn read<W: Write>(&self, id: &str, key: &SecretKey, writer: &mut W) -> BlobResult<u64> {
        let path = self.path_with_extension(id, BLOB_EXTENSION)?;
        let mut reader = BufReader::new(File::open(path)?);
        decrypt_stream(key, id, &mut reader, writer)
    }

    pub fn delete(&self, id: &str) -> BlobResult<()> {
        let path = self.path_with_extension(id, BLOB_EXTENSION)?;
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn list_ids(&self) -> BlobResult<Vec<String>> {
        let mut ids = Vec::new();
        for entry in fs::read_dir(&self.directory)? {
            let path = entry?.path();
            let is_blob = path.extension().is_some_and(|ext| ext == BLOB_EXTENSION);
            let id = path.file_stem().and_then(|stem| stem.to_str());
            if let Some(id) = id.filter(|id| is_blob && is_valid_blob_id(id)) {
                ids.push(id.to_string());
            }
        }
        ids.sort();
        Ok(ids)
    }

    pub fn remove_temporary_files(&self) -> BlobResult<()> {
        for entry in fs::read_dir(&self.directory)? {
            let path = entry?.path();
            if path
                .extension()
                .is_some_and(|ext| ext == TEMPORARY_EXTENSION)
            {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }
}

fn write_encrypted<R: Read>(
    path: &Path,
    id: &str,
    key: &SecretKey,
    reader: &mut R,
) -> BlobResult<u64> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(OWNER_ONLY_FILE_MODE)
        .open(path)?;
    let mut writer = BufWriter::new(file);
    let size = encrypt_stream(key, id, reader, &mut writer)?;
    writer.flush()?;
    writer.get_ref().sync_all()?;
    Ok(size)
}
