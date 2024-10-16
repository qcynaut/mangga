/// Error
///
/// Represents an error in the database
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// MongoDB error
    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),
    /// Init error
    #[error("Init error: {0}")]
    Init(String),
}

/// Result
///
/// Alias for `Result<T, Error>`
pub type Result<T> = std::result::Result<T, Error>;
