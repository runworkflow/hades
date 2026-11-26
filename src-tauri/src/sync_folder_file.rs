use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

const FILE_NAME: &str = "sync-folder.txt";
const TEMPORARY_FILE_NAME: &str = "sync-folder.txt.tmp";
const OWNER_ONLY_FILE_MODE: u32 = 0o600;

pub fn load(data_directory: &Path) -> Option<PathBuf> {
    let text = fs::read_to_string(data_directory.join(FILE_NAME)).ok()?;
    let path = PathBuf::from(text.trim_end_matches('\n'));
    path.is_absolute().then_some(path)
}

pub fn save(data_directory: &Path, folder: &Path) -> std::io::Result<()> {
    let temporary_path = data_directory.join(TEMPORARY_FILE_NAME);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(OWNER_ONLY_FILE_MODE)
        .open(&temporary_path)?;
    file.write_all(folder.to_string_lossy().as_bytes())?;
    file.sync_all()?;
    fs::rename(&temporary_path, data_directory.join(FILE_NAME))
}

pub fn remove(data_directory: &Path) -> std::io::Result<()> {
    match fs::remove_file(data_directory.join(FILE_NAME)) {
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => Err(error),
        _ => Ok(()),
    }
}
