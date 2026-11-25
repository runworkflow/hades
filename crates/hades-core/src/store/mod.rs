mod envelope;
mod error;
mod schema;
mod sqlite_store;
mod stored_row;

pub use envelope::{Envelope, EnvelopeKind};
pub use error::{StoreError, StoreResult};
pub use sqlite_store::SqliteStore;

pub trait Store {
    fn put_envelope(&self, envelope: &Envelope) -> StoreResult<()>;
    fn get_envelope(&self, id: &str) -> StoreResult<Option<Envelope>>;
    fn list_envelopes(&self, kind: EnvelopeKind) -> StoreResult<Vec<Envelope>>;
}
