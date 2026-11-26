use std::io::Write;

use hades_core::{RecoveryKey, VaultError};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use zeroize::Zeroizing;

use crate::atomic_file::{AtomicWriteError, write_atomically};
use crate::command_error::CommandError;

const SUGGESTED_FILE_NAME: &str = "Hades recovery key.txt";

fn unwritable_file_error() -> CommandError {
    CommandError::new(
        "file_unwritable",
        "Hades could not save the file there.".to_string(),
    )
}

fn file_text(recovery_key: &RecoveryKey) -> Zeroizing<String> {
    let key_text = recovery_key.to_display_text();
    Zeroizing::new(format!(
        "Hades recovery key\n\n{}\n\nIf you forget your password, this key is the only way back into your vault.\nKeep this file somewhere safe and private. Anyone who has it and your vault files can open your vault.\n",
        key_text.as_str()
    ))
}

#[tauri::command(async)]
pub fn save_recovery_key(app: AppHandle, recovery_key: String) -> Result<bool, CommandError> {
    let recovery_key = Zeroizing::new(recovery_key);
    let parsed =
        RecoveryKey::parse(&recovery_key).map_err(|_: VaultError| CommandError::internal())?;
    let Some(chosen) = app
        .dialog()
        .file()
        .set_title("Save recovery key")
        .set_file_name(SUGGESTED_FILE_NAME)
        .blocking_save_file()
    else {
        return Ok(false);
    };
    let destination = chosen.into_path().map_err(|_| unwritable_file_error())?;
    let text = file_text(&parsed);

    write_atomically(&destination, |writer| writer.write_all(text.as_bytes())).map_err(
        |error| match error {
            AtomicWriteError::Io | AtomicWriteError::Write(_) => unwritable_file_error(),
        },
    )?;
    Ok(true)
}
