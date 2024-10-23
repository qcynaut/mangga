/// Error
///
/// Represents an error in the database
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// MongoDB error
    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),
    /// Not found error
    #[error("Not found")]
    NotFound,
    /// Init error
    #[error("Init error: {0}")]
    Init(String),
}

impl Error {
    /// Check if error is conflict error
    pub fn is_conflict(&self) -> bool {
        match self {
            Error::MongoDB(mongo_error) => {
                let code = match mongo_error.kind.as_ref() {
                    mongodb::error::ErrorKind::Command(command_error) => Some(command_error.code),
                    mongodb::error::ErrorKind::InsertMany(mongodb::error::InsertManyError {
                        write_concern_error: Some(wc_error),
                        ..
                    }) => Some(wc_error.code),
                    mongodb::error::ErrorKind::Write(e) => match e {
                        mongodb::error::WriteFailure::WriteConcernError(wc_error) => Some(wc_error.code),
                        mongodb::error::WriteFailure::WriteError(w_error) => Some(w_error.code),
                        _ => None,
                    }
                    _ => None,
                };
                if let Some(code) = code {
                    code == 11000 || code == 40 || code == 112
                } else {
                    false
                }
            }
            _ => false
        }
    }
}

/// Result
///
/// Alias for `Result<T, Error>`
pub type Result<T> = std::result::Result<T, Error>;
