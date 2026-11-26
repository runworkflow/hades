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
use super::sync_baseline::SyncBaseline;
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

const REPLACE_ENVELOPE: &str = "
    INSERT OR REPLACE INTO envelopes (
        id, kind, version, is_deleted, modified_at,
        key_nonce, key_ciphertext, nonce, ciphertext
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
";

const REPLACE_VAULT_SETTINGS: &str = "
    INSERT OR REPLACE INTO vault_settings (
        id, vault_id, settings_version, created_at, kdf_salt, kdf_memory_kib, kdf_iterations, kdf_lanes,
        password_key_nonce, password_key_ciphertext,
        recovery_key_nonce, recovery_key_ciphertext,
        biometric_key_blob, biometric_public_key,
        biometric_key_nonce, biometric_key_ciphertext
    ) VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
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

fn upsert_envelope(connection: &Connection, envelope: &Envelope) -> StoreResult<()> {
    let version = i64::try_from(envelope.version).map_err(|_| StoreError::CorruptRecord)?;
    let changed_rows = connection.execute(
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

impl Store for SqliteStore {
    fn put_envelope(&self, envelope: &Envelope) -> StoreResult<()> {
        upsert_envelope(&self.connection, envelope)
    }

    fn put_envelopes(&self, envelopes: &[Envelope]) -> StoreResult<()> {
        let transaction = self.connection.unchecked_transaction()?;
        for envelope in envelopes {
            upsert_envelope(&transaction, envelope)?;
        }
        transaction.commit()?;
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

    fn list_all_envelopes(&self) -> StoreResult<Vec<Envelope>> {
        let query = format!("SELECT {SELECT_COLUMNS} FROM envelopes ORDER BY id");
        let mut statement = self.connection.prepare(&query)?;
        let stored_rows = statement
            .query_map([], StoredRow::from_row)?
            .collect::<Result<Vec<_>, _>>()?;
        stored_rows.into_iter().map(Envelope::try_from).collect()
    }

    fn replace_envelope(&self, envelope: &Envelope) -> StoreResult<()> {
        let version = i64::try_from(envelope.version).map_err(|_| StoreError::CorruptRecord)?;
        self.connection.execute(
            REPLACE_ENVELOPE,
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
        Ok(())
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
        let biometric = settings.biometric.as_ref();
        let settings_version =
            i64::try_from(settings.settings_version).map_err(|_| StoreError::CorruptRecord)?;
        self.connection.execute(
            REPLACE_VAULT_SETTINGS,
            params![
                settings.vault_id,
                settings_version,
                settings.created_at,
                settings.kdf_salt.as_bytes(),
                settings.kdf_params.memory_kib,
                settings.kdf_params.iterations,
                settings.kdf_params.lanes,
                settings.password_wrapped_key.0.nonce,
                settings.password_wrapped_key.0.ciphertext,
                settings.recovery_wrapped_key.0.nonce,
                settings.recovery_wrapped_key.0.ciphertext,
                biometric.map(|b| b.key_blob.as_slice()),
                biometric.map(|b| b.ephemeral_public_key.as_slice()),
                biometric.map(|b| b.wrapped_key.0.nonce),
                biometric.map(|b| b.wrapped_key.0.ciphertext.as_slice()),
            ],
        )?;
        Ok(())
    }

    fn get_sync_baseline(&self, id: &str) -> StoreResult<Option<SyncBaseline>> {
        let stored: Option<(i64, Vec<u8>)> = self
            .connection
            .query_row(
                "SELECT version, payload_nonce FROM sync_baselines WHERE id = ?1",
                params![id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        stored
            .map(|(version, nonce)| {
                Ok(SyncBaseline {
                    version: u64::try_from(version).map_err(|_| StoreError::CorruptRecord)?,
                    payload_nonce: nonce.try_into().map_err(|_| StoreError::CorruptRecord)?,
                })
            })
            .transpose()
    }

    fn put_sync_baseline(&self, id: &str, baseline: &SyncBaseline) -> StoreResult<()> {
        let version = i64::try_from(baseline.version).map_err(|_| StoreError::CorruptRecord)?;
        self.connection.execute(
            "INSERT OR REPLACE INTO sync_baselines (id, version, payload_nonce)
             VALUES (?1, ?2, ?3)",
            params![id, version, baseline.payload_nonce],
        )?;
        Ok(())
    }

    fn clear_sync_baselines(&self) -> StoreResult<()> {
        self.connection.execute("DELETE FROM sync_baselines", [])?;
        Ok(())
    }
}
