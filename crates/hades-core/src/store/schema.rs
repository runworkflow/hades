use rusqlite::Connection;

use super::error::{StoreError, StoreResult};

const CURRENT_SCHEMA_VERSION: i64 = 1;

const CREATE_SCHEMA_VERSION_1: &str = "
    BEGIN;
    CREATE TABLE envelopes (
        id TEXT PRIMARY KEY NOT NULL,
        kind TEXT NOT NULL,
        version INTEGER NOT NULL,
        is_deleted INTEGER NOT NULL DEFAULT 0,
        modified_at INTEGER NOT NULL,
        key_nonce BLOB NOT NULL,
        key_ciphertext BLOB NOT NULL,
        nonce BLOB NOT NULL,
        ciphertext BLOB NOT NULL
    ) STRICT;
    CREATE INDEX envelopes_by_kind ON envelopes (kind, is_deleted);
    PRAGMA user_version = 1;
    COMMIT;
";

pub fn prepare_database(connection: &Connection) -> StoreResult<()> {
    connection.pragma_update(None, "secure_delete", "ON")?;
    let stored_version: i64 =
        connection.pragma_query_value(None, "user_version", |row| row.get(0))?;

    match stored_version {
        0 => connection.execute_batch(CREATE_SCHEMA_VERSION_1)?,
        CURRENT_SCHEMA_VERSION => {}
        _ => return Err(StoreError::UnsupportedSchema),
    }
    Ok(())
}
