use rusqlite::Connection;

use super::error::{StoreError, StoreResult};

const CURRENT_SCHEMA_VERSION: i64 = 5;

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

const UPGRADE_TO_SCHEMA_VERSION_2: &str = "
    BEGIN;
    CREATE TABLE vault_settings (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        created_at INTEGER NOT NULL,
        kdf_salt BLOB NOT NULL,
        kdf_memory_kib INTEGER NOT NULL,
        kdf_iterations INTEGER NOT NULL,
        kdf_lanes INTEGER NOT NULL,
        password_key_nonce BLOB NOT NULL,
        password_key_ciphertext BLOB NOT NULL,
        recovery_key_nonce BLOB NOT NULL,
        recovery_key_ciphertext BLOB NOT NULL
    ) STRICT;
    PRAGMA user_version = 2;
    COMMIT;
";

const UPGRADE_TO_SCHEMA_VERSION_3: &str = "
    BEGIN;
    ALTER TABLE vault_settings ADD COLUMN biometric_key_blob BLOB;
    ALTER TABLE vault_settings ADD COLUMN biometric_public_key BLOB;
    ALTER TABLE vault_settings ADD COLUMN biometric_key_nonce BLOB;
    ALTER TABLE vault_settings ADD COLUMN biometric_key_ciphertext BLOB;
    PRAGMA user_version = 3;
    COMMIT;
";

const UPGRADE_TO_SCHEMA_VERSION_4: &str = "
    BEGIN;
    ALTER TABLE vault_settings ADD COLUMN vault_id TEXT;
    ALTER TABLE vault_settings ADD COLUMN settings_version INTEGER NOT NULL DEFAULT 1;
    UPDATE vault_settings SET vault_id = lower(hex(randomblob(16)));
    CREATE TABLE sync_baselines (
        id TEXT PRIMARY KEY NOT NULL,
        version INTEGER NOT NULL
    ) STRICT;
    PRAGMA user_version = 4;
    COMMIT;
";

const UPGRADE_TO_SCHEMA_VERSION_5: &str = "
    BEGIN;
    DROP TABLE sync_baselines;
    CREATE TABLE sync_baselines (
        id TEXT PRIMARY KEY NOT NULL,
        version INTEGER NOT NULL,
        payload_nonce BLOB NOT NULL
    ) STRICT;
    PRAGMA user_version = 5;
    COMMIT;
";

pub fn prepare_database(connection: &Connection) -> StoreResult<()> {
    connection.pragma_update(None, "secure_delete", "ON")?;
    let mut version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;

    if version > CURRENT_SCHEMA_VERSION {
        return Err(StoreError::UnsupportedSchema);
    }
    if version == 0 {
        connection.execute_batch(CREATE_SCHEMA_VERSION_1)?;
        version = 1;
    }
    if version == 1 {
        connection.execute_batch(UPGRADE_TO_SCHEMA_VERSION_2)?;
        version = 2;
    }
    if version == 2 {
        connection.execute_batch(UPGRADE_TO_SCHEMA_VERSION_3)?;
        version = 3;
    }
    if version == 3 {
        connection.execute_batch(UPGRADE_TO_SCHEMA_VERSION_4)?;
        version = 4;
    }
    if version == 4 {
        connection.execute_batch(UPGRADE_TO_SCHEMA_VERSION_5)?;
    }
    Ok(())
}
