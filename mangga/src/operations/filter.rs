use crate::traits::Field;
use bson::{doc, Bson, Document};

/// AsFilter
/// 
/// Allows expression to be used as filter
pub trait AsFilter {
    /// Get the expression as filter
    fn as_filter(self) -> Document;
}

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

    /// Create or from query
    pub fn or<FF, VV>(self, other: Query<FF, VV>) -> Filter
    where
        F: Field,
        FF: Field,
        V: Into<Bson>,
        VV: Into<Bson>,
    {
        Filter::new().or(self).or(other)
    }

    /// Create and from query
    pub fn and<FF, VV>(self, other: Query<FF, VV>) -> Filter
    where
        F: Field,
        FF: Field,
        V: Into<Bson>,
        VV: Into<Bson>,
    {
        Filter::new().and(self).and(other)
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

impl<F, V> AsFilter for Query<F, V>
where
    F: Field,
    V: Into<Bson>,
{
    fn as_filter(self) -> Document {
        self.into()
    }
}

/// Queryable
///
/// Allows a certain field to be queried
pub trait Queryable: Field + Sized {
    /// Create `eq` query
    fn eq<V: Into<Self::Type>>(self, value: V) -> Query<Self, Self::Type> {
        Query::new(Operator::Eq, value.into())
    }

    /// Create `lt` query
    fn lt<V: Into<Self::Type>>(self, value: V) -> Query<Self, Self::Type> {
        Query::new(Operator::Lt, value.into())
    }

    /// Create `gt` query
    fn gt<V: Into<Self::Type>>(self, value: V) -> Query<Self, Self::Type> {
        Query::new(Operator::Gt, value.into())
    }

    /// Create `lte` query
    fn lte<V: Into<Self::Type>>(self, value: V) -> Query<Self, Self::Type> {
        Query::new(Operator::Lte, value.into())
    }

    /// Create `gte` query
    fn gte<V: Into<Self::Type>>(self, value: V) -> Query<Self, Self::Type> {
        Query::new(Operator::Gte, value.into())
    }

    /// Create `ne` query
    fn ne<V: Into<Self::Type>>(self, value: V) -> Query<Self, Self::Type> {
        Query::new(Operator::Ne, value.into())
    }

    /// Create `in` query
    fn is_in<T: Into<Self::Type>, V: IntoIterator<Item = T>>(
        self,
        value: V,
    ) -> Query<Self, Vec<Self::Type>> {
        Query::new(Operator::In, value.into_iter().map(Into::into).collect())
    }

    /// Create `nin` query
    fn nin<T: Into<Self::Type>,V: IntoIterator<Item = T>>(self, value: V) -> Query<Self, Vec<Self::Type>> {
        Query::new(Operator::Nin, value.into_iter().map(Into::into).collect())
    }
}

impl<T> Queryable for T where T: Field {}

/// Filter
pub enum Filter {
    Or(Vec<Document>),
    And(Vec<Document>),
    None,
}

impl Filter {
    /// Create a new filter
    pub fn new() -> Self {
        Self::None
    }

    /// Add `or` filter
    pub fn or<F: Into<Document>>(mut self, filter: F) -> Self {
        match self {
            Self::None => Self::Or(vec![filter.into()]),
            Self::Or(ref mut filters) => {
                filters.push(filter.into());
                self
            }
            Self::And(ref mut filters) => {
                filters.push(filter.into());
                self
            }
        }
    }

    /// Add `and` filter
    pub fn and<F: Into<Document>>(mut self, filter: F) -> Self {
        match self {
            Self::None => Self::And(vec![filter.into()]),
            Self::Or(ref mut filters) => {
                filters.push(filter.into());
                self
            }
            Self::And(ref mut filters) => {
                filters.push(filter.into());
                self
            }
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Filter> for Document {
    fn from(value: Filter) -> Self {
        match value {
            Filter::Or(filters) => doc! {
                "$or": filters
            },
            Filter::And(filters) => doc! {
                "$and": filters
            },
            Filter::None => doc! {},
        }
    }
}

impl AsFilter for Filter {
    fn as_filter(self) -> Document {
        self.into()
    }
}

impl AsFilter for () {
    fn as_filter(self) -> Document {
        doc! {}
    }
}