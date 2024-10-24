use super::Model;
use crate::operations::{DeleteOne, InsertOne};
use bson::doc;
use serde::Serialize;

/// Insertable
///
/// Allows to insert current model
pub trait Insertable: Model + Serialize {
    /// Insert current model
    fn insert(&self) -> InsertOne<Self> {
        InsertOne::new(self)
    }
}

/// Deletable
///
/// Allows to delete current model
pub trait Deletable: Model + Serialize {
    /// Delete current model
    fn delete(&self) -> DeleteOne<Self> {
        DeleteOne::new(doc! {"_id": self.id().into()})
    }
}

impl<T> Insertable for T where T: Model + Serialize {}

impl<T> Deletable for T where T: Model + Serialize {}
