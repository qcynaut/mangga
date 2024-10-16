pub use error::*;
pub use id::{is_id, ID};
use std::pin::Pin;

mod error;
mod id;

/// BoxFut
///
/// Type alias for futures
pub(crate) type BoxFut<T> = Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>;
