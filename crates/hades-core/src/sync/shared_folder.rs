use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

use crate::blobs::FileBlobStore;
use crate::records::is_valid_record_id;
use crate::store::Envelope;

use super::record_file::{decode_envelope, encode_envelope};
use super::sync_error::{SyncError, SyncResult};
use super::synced_settings::{SyncedSettings, decode_settings, encode_settings};

const VAULT_DIRECTORY_NAME: &str = "Hades Vault";
const RECORDS_DIRECTORY_NAME: &str = "records";
const BLOBS_DIRECTORY_NAME: &str = "blobs";
const SETTINGS_FILE_NAME: &str = "vault.json";
const RECORD_EXTENSION: &str = "json";
const BLOB_EXTENSION: &str = "blob";
const MAXIMUM_SMALL_FILE_BYTES: u64 = 4 * 1024 * 1024;
const OWNER_ONLY_DIRECTORY_MODE: u32 = 0o700;
const OWNER_ONLY_FILE_MODE: u32 = 0o600;

#[derive(Debug, Default)]
pub struct RecordScan {
    pub envelopes: Vec<Envelope>,
    pub skipped_files: Vec<String>,
}

pub struct SharedFolder {
    records_directory: PathBuf,
    blobs_directory: PathBuf,
    settings_path: PathBuf,
}

fn create_directory(path: &Path) -> SyncResult<()> {
    DirBuilder::new()
        .recursive(true)
        .mode(OWNER_ONLY_DIRECTORY_MODE)
        .create(path)?;
    Ok(())
}

fn write_atomically(path: &Path, bytes: &[u8]) -> SyncResult<()> {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(SyncError::InvalidName)?;
    let temporary_path = path.with_file_name(format!(".{name}.tmp"));
    let outcome = (|| -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(OWNER_ONLY_FILE_MODE)
            .open(&temporary_path)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        fs::rename(&temporary_path, path)
    })();
    if outcome.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }
    Ok(outcome?)
}

fn read_small_file(path: &Path) -> SyncResult<Vec<u8>> {
    if !fs::symlink_metadata(path)?.is_file() {
        return Err(SyncError::DamagedFile);
    }
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAXIMUM_SMALL_FILE_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAXIMUM_SMALL_FILE_BYTES {
        return Err(SyncError::DamagedFile);
    }
    Ok(bytes)
}

fn file_stem_with_extension<'a>(path: &'a Path, extension: &str) -> Option<&'a str> {
    let has_extension = path.extension().is_some_and(|found| found == extension);
    let stem = path.file_stem().and_then(|stem| stem.to_str())?;
    (has_extension && is_valid_record_id(stem)).then_some(stem)
}

fn read_record_file(path: &Path, expected_id: &str) -> SyncResult<Envelope> {
    let envelope = decode_envelope(&read_small_file(path)?)?;
    if envelope.id != expected_id {
        return Err(SyncError::DamagedFile);
    }
    Ok(envelope)
}

fn copy_atomically(source: &Path, destination: &Path, temporary: &Path) -> SyncResult<()> {
    let outcome = fs::copy(source, temporary).and_then(|_| {
        File::open(temporary)?.sync_all()?;
        fs::rename(temporary, destination)
    });
    if outcome.is_err() {
        let _ = fs::remove_file(temporary);
    }
    Ok(outcome?)
}

impl SharedFolder {
    pub fn open(root: &Path) -> SyncResult<Self> {
        let vault_directory = root.join(VAULT_DIRECTORY_NAME);
        let records_directory = vault_directory.join(RECORDS_DIRECTORY_NAME);
        let blobs_directory = vault_directory.join(BLOBS_DIRECTORY_NAME);
        create_directory(&records_directory)?;
        create_directory(&blobs_directory)?;
        Ok(Self {
            records_directory,
            blobs_directory,
            settings_path: vault_directory.join(SETTINGS_FILE_NAME),
        })
    }

    pub fn open_existing(root: &Path) -> SyncResult<Self> {
        let settings_path = root.join(VAULT_DIRECTORY_NAME).join(SETTINGS_FILE_NAME);
        if !fs::symlink_metadata(&settings_path).is_ok_and(|metadata| metadata.is_file()) {
            return Err(SyncError::FolderHasNoVault);
        }
        Self::open(root)
    }

    pub fn read_records(&self) -> SyncResult<RecordScan> {
        let mut scan = RecordScan::default();
        for entry in fs::read_dir(&self.records_directory)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if file_name.starts_with('.') {
                continue;
            }
            let Some(id) = file_stem_with_extension(&path, RECORD_EXTENSION) else {
                scan.skipped_files.push(file_name);
                continue;
            };
            match read_record_file(&path, id) {
                Ok(envelope) => scan.envelopes.push(envelope),
                Err(_) => scan.skipped_files.push(file_name),
            }
        }
        scan.envelopes
            .sort_by(|first, second| first.id.cmp(&second.id));
        scan.skipped_files.sort();
        Ok(scan)
    }

    pub fn write_record(&self, envelope: &Envelope) -> SyncResult<()> {
        if !is_valid_record_id(&envelope.id) {
            return Err(SyncError::InvalidName);
        }
        let path = self
            .records_directory
            .join(format!("{}.{RECORD_EXTENSION}", envelope.id));
        write_atomically(&path, &encode_envelope(envelope)?)
    }

    pub fn read_settings(&self) -> SyncResult<Option<SyncedSettings>> {
        if fs::symlink_metadata(&self.settings_path).is_err() {
            return Ok(None);
        }
        decode_settings(&read_small_file(&self.settings_path)?).map(Some)
    }

    pub fn write_settings(&self, settings: &SyncedSettings) -> SyncResult<()> {
        write_atomically(&self.settings_path, &encode_settings(settings)?)
    }

    pub fn list_blob_ids(&self) -> SyncResult<Vec<String>> {
        let mut ids = Vec::new();
        for entry in fs::read_dir(&self.blobs_directory)? {
            let path = entry?.path();
            if let Some(id) = file_stem_with_extension(&path, BLOB_EXTENSION) {
                ids.push(id.to_string());
            }
        }
        ids.sort();
        Ok(ids)
    }

    pub fn push_blob(&self, local: &FileBlobStore, id: &str) -> SyncResult<()> {
        let source = local.final_path(id).map_err(|_| SyncError::InvalidName)?;
        let destination = self.blobs_directory.join(format!("{id}.{BLOB_EXTENSION}"));
        if destination.exists() {
            return Ok(());
        }
        let temporary = self.blobs_directory.join(format!(".{id}.tmp"));
        copy_atomically(&source, &destination, &temporary)
    }

    pub fn pull_blob(&self, local: &FileBlobStore, id: &str) -> SyncResult<()> {
        let destination = local.final_path(id).map_err(|_| SyncError::InvalidName)?;
        if destination.exists() {
            return Ok(());
        }
        let source = self.blobs_directory.join(format!("{id}.{BLOB_EXTENSION}"));
        let temporary = local
            .temporary_path(id)
            .map_err(|_| SyncError::InvalidName)?;
        copy_atomically(&source, &destination, &temporary)
    }
}
