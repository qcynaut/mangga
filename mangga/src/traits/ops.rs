use super::{AsFilter, Dsl, Model};
use crate::operations::{
    Count,
    DeleteMany,
    DeleteOne,
    FindMany,
    FindOne,
    InsertMany,
    InsertOne,
    UpdateMany,
    UpdateOne,
};
use bson::Bson;

/// Ops
///
/// Represents the operations of the model
pub trait Ops<M: Model> {
    /// Insert one model
    fn insert_one<'a>(&self, model: &'a M) -> InsertOne<'a, M>;

    /// Insert many models
    fn insert_many<T: IntoIterator<Item = M>>(&self, models: T) -> InsertMany<M>;

    /// Find one model
    fn find_one<F: AsFilter>(&self, filter: F) -> FindOne<M>;

    /// Find many models
    fn find_many<F: AsFilter>(&self, filter: F) -> FindMany<M>;

    /// Delete one model
    fn delete_one<F: AsFilter>(&self, filter: F) -> DeleteOne<M>;

    /// Delete many models
    fn delete_many<F: AsFilter>(&self, filter: F) -> DeleteMany<M>;

    /// Update one model
    fn update_one<F: AsFilter>(&self, filter: F, update: Vec<(String, Bson)>) -> UpdateOne<M>;

    /// Update many models
    fn update_many<F: AsFilter>(&self, filter: F, update: Vec<(String, Bson)>) -> UpdateMany<M>;

    /// Count the number of models
    fn count<F: AsFilter>(&self, filter: F) -> Count<M>;
}

impl<M, D> Ops<M> for D
where
    M: Model,
    D: Dsl<M>,
{
    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn insert_many<T: IntoIterator<Item = M>>(&self, models: T) -> InsertMany<M> {
        InsertMany::new(models.into_iter().collect())
    }

    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn insert_one<'a>(&self, model: &'a M) -> InsertOne<'a, M> {
        InsertOne::new(model)
    }

    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn find_one<F: AsFilter>(&self, filter: F) -> FindOne<M> {
        FindOne::new(filter.as_filter())
    }

    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn find_many<F: AsFilter>(&self, filter: F) -> FindMany<M> {
        FindMany::new(filter.as_filter())
    }

    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn delete_one<F: AsFilter>(&self, filter: F) -> DeleteOne<M> {
        DeleteOne::new(filter.as_filter())
    }

    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn delete_many<F: AsFilter>(&self, filter: F) -> DeleteMany<M> {
        DeleteMany::new(filter.as_filter())
    }

    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn update_one<F: AsFilter>(&self, filter: F, update: Vec<(String, Bson)>) -> UpdateOne<M> {
        UpdateOne::new(filter.as_filter(), update)
    }

    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn update_many<F: AsFilter>(&self, filter: F, update: Vec<(String, Bson)>) -> UpdateMany<M> {
        UpdateMany::new(filter.as_filter(), update)
    }

    #[tracing::instrument(skip_all, level = tracing::Level::DEBUG)]
    fn count<F: AsFilter>(&self, filter: F) -> Count<M> {
        Count::new(filter.as_filter())
    }
}
