pub use error::*;
pub use id::{is_id, ID};
pub use datetime::DateTime;
use std::pin::Pin;

mod error;
mod id;
mod datetime;

/// BoxFut
///
/// Type alias for futures
pub(crate) type BoxFut<T> = Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>;
