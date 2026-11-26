use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use hades_core::clean_file_name;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use crate::app_vault::AppVault;
use crate::atomic_file::{AtomicWriteError, write_atomically};
use crate::clock::current_unix_millis;
use crate::command_error::CommandError;
use crate::item_commands::ItemSummaryResponse;
use crate::vault_commands::{lock_session, record_activity};

fn unreadable_file_error() -> CommandError {
    CommandError::new(
        "file_unreadable",
        "Hades could not read that file.".to_string(),
    )
}

fn unwritable_file_error() -> CommandError {
    CommandError::new(
        "file_unwritable",
        "Hades could not save the file there.".to_string(),
    )
}

#[tauri::command(async)]
pub fn import_file(
    app: AppHandle,
    vault: State<'_, AppVault>,
    folder_id: String,
) -> Result<Option<ItemSummaryResponse>, CommandError> {
    record_activity(&vault);
    let Some(chosen) = app
        .dialog()
        .file()
        .set_title("Choose a file to add")
        .blocking_pick_file()
    else {
        return Ok(None);
    };
    let path = chosen.into_path().map_err(|_| unreadable_file_error())?;
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();

    let file = File::open(&path).map_err(|_| unreadable_file_error())?;
    let is_regular_file = file.metadata().is_ok_and(|metadata| metadata.is_file());
    if !is_regular_file {
        return Err(unreadable_file_error());
    }

    let mut reader = BufReader::new(file);
    let summary = lock_session(&vault)?.import_file_item(
        &vault.blobs,
        &folder_id,
        &file_name,
        &mut reader,
        current_unix_millis(),
    )?;
    Ok(Some(summary.into()))
}

#[tauri::command(async)]
pub fn export_file(
    app: AppHandle,
    vault: State<'_, AppVault>,
    item_id: String,
) -> Result<bool, CommandError> {
    record_activity(&vault);
    let stored_name = lock_session(&vault)?.read_item_field(&item_id, "fileName")?;
    let Some(chosen) = app
        .dialog()
        .file()
        .set_title("Export file")
        .set_file_name(clean_file_name(&stored_name))
        .blocking_save_file()
    else {
        return Ok(false);
    };
    let destination = chosen.into_path().map_err(|_| unwritable_file_error())?;

    write_decrypted_file(&vault, &item_id, &destination)?;
    Ok(true)
}

fn write_decrypted_file(
    vault: &AppVault,
    item_id: &str,
    destination: &Path,
) -> Result<(), CommandError> {
    write_atomically(destination, |writer| {
        lock_session(vault)?
            .export_file_item(&vault.blobs, item_id, writer)
            .map(|_| ())
            .map_err(CommandError::from)
    })
    .map_err(|error| match error {
        AtomicWriteError::Io => unwritable_file_error(),
        AtomicWriteError::Write(command_error) => command_error,
    })
}
