use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

const OWNER_ONLY_FILE_MODE: u32 = 0o600;

pub enum AtomicWriteError<E> {
    Io,
    Write(E),
}

fn temporary_path_for(destination: &Path) -> PathBuf {
    let name = destination
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    destination.with_file_name(format!(".{name}.hades-export"))
}

pub fn write_atomically<T, E>(
    destination: &Path,
    write: impl FnOnce(&mut BufWriter<File>) -> Result<T, E>,
) -> Result<T, AtomicWriteError<E>> {
    let temporary_path = temporary_path_for(destination);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(OWNER_ONLY_FILE_MODE)
        .open(&temporary_path)
        .map_err(|_| AtomicWriteError::Io)?;

    let mut writer = BufWriter::new(file);
    let outcome = write(&mut writer)
        .map_err(AtomicWriteError::Write)
        .and_then(|value| {
            writer.flush().map_err(|_| AtomicWriteError::Io)?;
            writer
                .get_ref()
                .sync_all()
                .map_err(|_| AtomicWriteError::Io)?;
            fs::rename(&temporary_path, destination).map_err(|_| AtomicWriteError::Io)?;
            Ok(value)
        });

    if outcome.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }
    outcome
}
