mod envelope;
mod error;
mod schema;
mod settings_row;
mod sqlite_store;
mod stored_row;
mod vault_settings;

pub use envelope::{Envelope, EnvelopeKind};
pub use error::{StoreError, StoreResult};
pub use sqlite_store::SqliteStore;
pub use vault_settings::{BiometricSettings, VaultSettings};

pub trait Store {
    fn put_envelope(&self, envelope: &Envelope) -> StoreResult<()>;
    fn put_envelopes(&self, envelopes: &[Envelope]) -> StoreResult<()>;
    fn get_envelope(&self, id: &str) -> StoreResult<Option<Envelope>>;
    fn list_envelopes(&self, kind: EnvelopeKind) -> StoreResult<Vec<Envelope>>;
    fn get_vault_settings(&self) -> StoreResult<Option<VaultSettings>>;
    fn put_vault_settings(&self, settings: &VaultSettings) -> StoreResult<()>;
}
