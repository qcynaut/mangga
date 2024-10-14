/// Error
///
/// The error type for mangga
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] mongodb::error::Error),
    #[error("Uninitialized database")]
    UninitializedDatabase,
    #[error("Initialization error")]
    InitializationError,
    #[error("Document not found")]
    DocumentNotFound,
    #[error(transparent)]
    BsonSerializationError(#[from] bson::ser::Error),
    #[error(transparent)]
    BsonDeserializationError(#[from] bson::de::Error),
}

/// Result
///
/// The result type for mangga
pub type Result<T> = std::result::Result<T, Error>;
