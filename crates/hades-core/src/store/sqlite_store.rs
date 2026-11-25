use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};

use super::Store;
use super::envelope::{Envelope, EnvelopeKind};
use super::error::{StoreError, StoreResult};
use super::schema::prepare_database;
use super::settings_row::{SELECT_SETTINGS_COLUMNS, StoredSettingsRow};
use super::stored_row::{SELECT_COLUMNS, StoredRow};
use super::vault_settings::VaultSettings;

const OWNER_ONLY_FILE_MODE: u32 = 0o600;

const UPSERT_ENVELOPE: &str = "
    INSERT INTO envelopes (
        id, kind, version, is_deleted, modified_at,
        key_nonce, key_ciphertext, nonce, ciphertext
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
    ON CONFLICT (id) DO UPDATE SET
        kind = excluded.kind,
        version = excluded.version,
        is_deleted = excluded.is_deleted,
        modified_at = excluded.modified_at,
        key_nonce = excluded.key_nonce,
        key_ciphertext = excluded.key_ciphertext,
        nonce = excluded.nonce,
        ciphertext = excluded.ciphertext
    WHERE excluded.version > envelopes.version
";

const REPLACE_VAULT_SETTINGS: &str = "
    INSERT OR REPLACE INTO vault_settings (
        id, created_at, kdf_salt, kdf_memory_kib, kdf_iterations, kdf_lanes,
        password_key_nonce, password_key_ciphertext,
        recovery_key_nonce, recovery_key_ciphertext
    ) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
";

pub struct SqliteStore {
    connection: Connection,
}

impl SqliteStore {
    pub fn open(path: &Path) -> StoreResult<Self> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .mode(OWNER_ONLY_FILE_MODE)
            .open(path)?;
        let connection = Connection::open(path)?;
        prepare_database(&connection)?;
        Ok(Self { connection })
    }
}

impl Store for SqliteStore {
    fn put_envelope(&self, envelope: &Envelope) -> StoreResult<()> {
        let version = i64::try_from(envelope.version).map_err(|_| StoreError::CorruptRecord)?;
        let changed_rows = self.connection.execute(
            UPSERT_ENVELOPE,
            params![
                envelope.id,
                envelope.kind.as_str(),
                version,
                envelope.is_deleted,
                envelope.modified_at,
                envelope.wrapped_key.0.nonce,
                envelope.wrapped_key.0.ciphertext,
                envelope.payload.nonce,
                envelope.payload.ciphertext,
            ],
        )?;
        if changed_rows == 0 {
            return Err(StoreError::StaleVersion);
        }
        Ok(())
    }

    fn get_envelope(&self, id: &str) -> StoreResult<Option<Envelope>> {
        let query = format!("SELECT {SELECT_COLUMNS} FROM envelopes WHERE id = ?1");
        self.connection
            .query_row(&query, params![id], StoredRow::from_row)
            .optional()?
            .map(Envelope::try_from)
            .transpose()
    }

    fn list_envelopes(&self, kind: EnvelopeKind) -> StoreResult<Vec<Envelope>> {
        let query = format!(
            "SELECT {SELECT_COLUMNS} FROM envelopes
             WHERE kind = ?1 AND is_deleted = 0 ORDER BY modified_at DESC"
        );
        let mut statement = self.connection.prepare(&query)?;
        let stored_rows = statement
            .query_map(params![kind.as_str()], StoredRow::from_row)?
            .collect::<Result<Vec<_>, _>>()?;
        stored_rows.into_iter().map(Envelope::try_from).collect()
    }

    fn get_vault_settings(&self) -> StoreResult<Option<VaultSettings>> {
        let query = format!("SELECT {SELECT_SETTINGS_COLUMNS} FROM vault_settings WHERE id = 1");
        self.connection
            .query_row(&query, [], StoredSettingsRow::from_row)
            .optional()?
            .map(VaultSettings::try_from)
            .transpose()
    }

    fn put_vault_settings(&self, settings: &VaultSettings) -> StoreResult<()> {
        self.connection.execute(
            REPLACE_VAULT_SETTINGS,
            params![
                settings.created_at,
                settings.kdf_salt.as_bytes(),
                settings.kdf_params.memory_kib,
                settings.kdf_params.iterations,
                settings.kdf_params.lanes,
                settings.password_wrapped_key.0.nonce,
                settings.password_wrapped_key.0.ciphertext,
                settings.recovery_wrapped_key.0.nonce,
                settings.recovery_wrapped_key.0.ciphertext,
            ],
        )?;
        Ok(())
    }
}
