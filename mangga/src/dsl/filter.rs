use super::{logicals::IntoLogical, query::AsQuery};
use crate::traits::{Expression, ManggaDoc};

/// Marker trait to indicate that the expression is a filter
pub trait IntoFilter {
    /// Build the filter
    fn into_filter(self) -> impl Expression;
}

/// Filter
pub struct Filter<T, M> {
    filter: T,
    _model: std::marker::PhantomData<M>,
}

impl<T, M> Filter<T, M> {
    pub fn new(filter: T) -> Self {
        Self {
            filter,
            _model: std::marker::PhantomData,
        }
    }
}

impl<T, M> AsQuery for Filter<T, M>
where
    T: IntoFilter,
    M: ManggaDoc,
{
    type Model = M;

    fn build(self) -> bson::Bson {
        self.filter.into_filter().build()
    }
}

#[derive(Debug, Clone)]
pub struct RawFilter {
    doc: bson::Document,
}

impl RawFilter {
    pub fn new(doc: bson::Document) -> Self {
        Self { doc }
    }
}

impl Expression for RawFilter {
    fn build(self) -> bson::Bson {
        bson::Bson::Document(self.doc)
    }
}

impl<M: ManggaDoc> IntoLogical for Filter<RawFilter, M> {
    fn into_logical(self) -> impl Expression {
        self.filter
    }
}

impl<M: ManggaDoc> AsQuery for Filter<RawFilter, M> {
    type Model = M;

    fn build(self) -> bson::Bson {
        self.filter.build()
    }
}
