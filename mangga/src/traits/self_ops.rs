use super::DatabaseName;
use crate::{db::get_database, operations::InsertOne, Result};
use bson::doc;
use serde::Serialize;

/// Insertable
///
/// Allows to insert current model
pub trait Insertable: DatabaseName + Serialize {
    /// Insert current model
    fn insert(&self) -> InsertOne<Self>;
}

/// Deletable
///
/// Allows to delete current model
pub trait Deletable {
    /// Delete current model
    async fn delete(&self) -> Result<()>;
}

impl<T> Insertable for T
where
    T: DatabaseName + Serialize,
{
    fn insert(&self) -> InsertOne<Self> {
        InsertOne {
            opts: None,
            data: self,
        }
    }
}

impl<T> Deletable for T
where
    T: DatabaseName + Serialize,
{
    async fn delete(&self) -> Result<()> {
        let db = get_database(T::DATABASE_NAME)?;
        let col = db.collection::<T>(T::MODEL_NAME);
        let id = self.id().into();
        col.delete_one(doc! {"_id": id}).await?;

        Ok(())
    }
}
