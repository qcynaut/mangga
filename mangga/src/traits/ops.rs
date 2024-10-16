use super::{Dsl, Model};
use crate::operations::{InsertMany, InsertOne};
use std::iter::Filter;

/// Ops
///
/// Represents the operations of the model
pub trait Ops<M: Model> {
    /// Insert one model
    fn insert_one<'a>(&self, model: &'a M) -> InsertOne<'a, M>;

    /// Insert many models
    fn insert_many<T: IntoIterator<Item = M>>(&self, models: T) -> InsertMany<M>;

    /// Find one model
    fn find_one<Pred>(&self, filter: Filter<Pred, M>);
}

impl<M, D> Ops<M> for D
where
    M: Model,
    D: Dsl<M>,
{
    fn insert_many<T: IntoIterator<Item = M>>(&self, models: T) -> InsertMany<M> {
        InsertMany {
            data: models.into_iter().collect(),
            opts: None,
        }
    }

    fn insert_one<'a>(&self, model: &'a M) -> InsertOne<'a, M> {
        InsertOne {
            data: model,
            opts: None,
        }
    }

    fn find_one<Pred>(&self, filter: Filter<Pred, M>) {}
}
