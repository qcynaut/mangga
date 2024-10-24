use crate::traits::{AsFilter, Field, Queryable};
use bson::{doc, Bson, Document};

// Define the query operators
pub enum Operator {
    Eq,
    Lt,
    Gt,
    Lte,
    Gte,
    Ne,
    In,
    Nin,
}

impl Operator {
    /// Get the operator as mongo operator
    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::Eq => "$eq",
            Operator::Lt => "$lt",
            Operator::Gt => "$gt",
            Operator::Lte => "$lte",
            Operator::Gte => "$gte",
            Operator::Ne => "$ne",
            Operator::In => "$in",
            Operator::Nin => "$nin",
        }
    }
}

/// Query
///
/// Represents a filter query
pub struct Query<F, V> {
    op: Operator,
    v: V,
    _field: std::marker::PhantomData<F>,
}

impl<F, V> Query<F, V> {
    /// Create a new query
    pub fn new(op: Operator, v: V) -> Self {
        Self {
            op,
            v,
            _field: std::marker::PhantomData,
        }
    }
}

impl<F, V> From<Query<F, V>> for Document
where
    F: Field,
    V: Into<Bson>,
{
    fn from(value: Query<F, V>) -> Self {
        let name = F::NAME;
        let op = value.op.as_str();
        let v = value.v;
        doc! {
            name: {op: v}
        }
    }
}

impl<F, V> From<Query<F,V>> for Bson
where
    F: Field,
    V: Into<Bson>,
{
    fn from(value: Query<F, V>) -> Self {
        let name = F::NAME;
        let op = value.op.as_str();
        let v = value.v;
        Bson::Document(doc! {
            name: {op: v}
        })
    }
}

impl<F, V> AsFilter for Query<F, V>
where
    F: Field,
    V: Into<Bson>,
{
    fn as_filter(self) -> Document {
        self.into()
    }
}


impl<T> Queryable for T where T: Field {}

impl AsFilter for () {
    fn as_filter(self) -> Document {
        doc! {}
    }
}

impl AsFilter for Document {
    fn as_filter(self) -> Document {
        self
    }
}