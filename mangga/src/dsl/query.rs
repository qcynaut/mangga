use super::operation::{delete, update};
use crate::{traits::ManggaDoc, AsUpdate};

pub mod count;
pub mod find;

/// AsQuery
///
/// Marker trait to indicate that the expression can be used as a query
pub trait AsQuery {
    type Model: ManggaDoc;

    /// Build the query
    fn build(self) -> bson::Bson;
}

/// ModelQuery
pub trait ModelQuery: AsQuery + Sized {
    type Model: ManggaDoc;

    /// Find single
    fn find_one(self) -> find::FindOne<<Self as ModelQuery>::Model> {
        find::FindOne::new(self.build())
    }

    /// Find many
    fn find(self) -> find::FindMany<<Self as ModelQuery>::Model> {
        find::FindMany::new(self.build())
    }

    /// Count
    fn count(self) -> count::Count<<Self as ModelQuery>::Model> {
        count::Count::new(self.build())
    }

    /// Delere Single
    fn delete_one(self) -> delete::DeleteOne<<Self as ModelQuery>::Model> {
        delete::DeleteOne::new(self.build().as_document().cloned().unwrap_or_default())
    }

    /// Delete Many
    fn delete_many(self) -> delete::DeleteMany<<Self as ModelQuery>::Model> {
        delete::DeleteMany::new(self.build().as_document().cloned().unwrap_or_default())
    }

    /// Update Single
    fn update_one<T>(self, doc: T) -> update::UpdateOne<<Self as ModelQuery>::Model, T>
    where
        T: AsUpdate,
    {
        update::UpdateOne::new(self.build().as_document().cloned().unwrap_or_default(), doc)
    }

    /// Update Many
    fn update_many<T>(self, doc: T) -> update::UpdateMany<<Self as ModelQuery>::Model, T>
    where
        T: AsUpdate,
    {
        update::UpdateMany::new(self.build().as_document().cloned().unwrap_or_default(), doc)
    }
}

impl<T> ModelQuery for T
where
    T: AsQuery,
    T::Model: ManggaDoc,
{
    type Model = T::Model;
}

impl<T> AsQuery for T
where
    T: ManggaDoc,
{
    type Model = T;
    fn build(self) -> bson::Bson {
        bson::bson!({})
    }
}
