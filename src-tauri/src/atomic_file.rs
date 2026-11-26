use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use hades_core::create_exclusive_temporary_file;

pub enum AtomicWriteError<E> {
    Io,
    Write(E),
}

pub fn write_atomically<T, E>(
    destination: &Path,
    write: impl FnOnce(&mut BufWriter<File>) -> Result<T, E>,
) -> Result<T, AtomicWriteError<E>> {
    let directory = destination.parent().ok_or(AtomicWriteError::Io)?;
    let label = destination
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .ok_or(AtomicWriteError::Io)?;
    let (file, temporary_path) =
        create_exclusive_temporary_file(directory, &label).map_err(|_| AtomicWriteError::Io)?;

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
