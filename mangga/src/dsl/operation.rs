use crate::{AsUpdate, ManggaDoc};

pub(crate) mod delete;
pub(crate) mod insert;
pub(crate) mod update;

/// BaseOperation
pub trait BaseOperation: ManggaDoc + Sized {
    /// Insert many
    fn insert<T>(self, docs: T) -> insert::InsertMany<Self>
    where
        T: IntoIterator<Item = Self::Model>,
    {
        insert::InsertMany::new(docs.into_iter().collect())
    }

    /// Insert one
    fn insert_one(self, doc: Self::Model) -> insert::InsertOne<Self> {
        insert::InsertOne::new(doc)
    }
}

impl<M: ManggaDoc> BaseOperation for M {}

/// AdvancedOperation
pub trait AdvancedOperation: ManggaDoc + Sized {
    /// Delete many
    fn delete(self) -> delete::DeleteMany<Self> {
        delete::DeleteMany::new(bson::Document::new())
    }

    /// Delete one
    fn delete_one(self) -> delete::DeleteOne<Self> {
        delete::DeleteOne::new(bson::Document::new())
    }

    /// Update many
    fn update<T>(self, doc: T) -> update::UpdateMany<Self, T>
    where
        T: AsUpdate,
    {
        update::UpdateMany::new(bson::Document::new(), doc)
    }

    /// Update one
    fn update_one<T>(self, doc: T) -> update::UpdateOne<Self, T>
    where
        T: AsUpdate,
    {
        update::UpdateOne::new(bson::Document::new(), doc)
    }
}

impl<M: ManggaDoc> AdvancedOperation for M {}
