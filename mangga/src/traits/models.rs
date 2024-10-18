use crate::types::ID;
use serde::{Deserialize, Serialize};

/// Model
///
/// Represents a struct of mangga model
pub trait Model: Default + Clone + Send + Sync + 'static {
    /// Name of the model
    const MODEL_NAME: &'static str;

    /// Database name of the model
    const DB_NAME: &'static str;

    /// Get id
    fn id(&self) -> impl Into<ID>;

    /// Get dsl of the model
    fn dsl() -> impl Dsl<Self>;
}

/// Dsl
///
/// Represents the dsl of the model
pub trait Dsl<T: Model> {}

/// Field
///
/// Represents a field of the model
pub trait Field {
    /// Model type of the field
    type Model: Model;

    /// Name of the field
    const NAME: &'static str;

    /// Type of the field
    type Type: Serialize + for<'de> Deserialize<'de>;
}
