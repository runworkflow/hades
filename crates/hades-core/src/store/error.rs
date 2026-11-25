use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("could not prepare the database file: {0}")]
    File(#[from] std::io::Error),
    #[error("a newer or equal version of this record is already stored")]
    StaleVersion,
    #[error("a stored record is damaged")]
    CorruptRecord,
    #[error("the database was created by a newer version of Hades")]
    UnsupportedSchema,
}

pub type StoreResult<T> = Result<T, StoreError>;
