mod envelope;
mod error;
mod schema;
mod settings_row;
mod sqlite_store;
mod stored_row;
mod sync_baseline;
mod vault_settings;

pub use envelope::{Envelope, EnvelopeKind};
pub use error::{StoreError, StoreResult};
pub use sqlite_store::SqliteStore;
pub use sync_baseline::SyncBaseline;
pub use vault_settings::{BiometricSettings, VaultSettings};

pub trait Store {
    fn put_envelope(&self, envelope: &Envelope) -> StoreResult<()>;
    fn put_envelopes(&self, envelopes: &[Envelope]) -> StoreResult<()>;
    fn get_envelope(&self, id: &str) -> StoreResult<Option<Envelope>>;
    fn list_envelopes(&self, kind: EnvelopeKind) -> StoreResult<Vec<Envelope>>;
    fn list_all_envelopes(&self) -> StoreResult<Vec<Envelope>>;
    fn replace_envelope(&self, envelope: &Envelope) -> StoreResult<()>;
    fn get_vault_settings(&self) -> StoreResult<Option<VaultSettings>>;
    fn put_vault_settings(&self, settings: &VaultSettings) -> StoreResult<()>;
    fn get_sync_baseline(&self, id: &str) -> StoreResult<Option<SyncBaseline>>;
    fn put_sync_baseline(&self, id: &str, baseline: &SyncBaseline) -> StoreResult<()>;
    fn clear_sync_baselines(&self) -> StoreResult<()>;
}
