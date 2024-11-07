use super::Field;
use crate::operations::{Operator, Query};
use bson::{bson, doc, Bson, Document};

/// AsFilter
///
/// Allows expression to be used as filter
pub trait AsFilter {
    /// Get the expression as filter
    fn as_filter(self) -> Document;
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
    fn is_in<V: IntoIterator<Item = Self::Type>>(
        self,
        value: V,
    ) -> Query<Self, Vec<Self::Type>> {
        Query::new(Operator::In, value.into_iter().collect())
    }

    /// Create `nin` query
    fn nin<T: Into<Self::Type>, V: IntoIterator<Item = T>>(
        self,
        value: V,
    ) -> Query<Self, Vec<Self::Type>> {
        Query::new(Operator::Nin, value.into_iter().map(Into::into).collect())
    }
}

/// SetAble
///
/// Allows a certain field to be set as update
pub trait SetAble: Field + Sized {
    /// Set current field
    fn set<V: Into<Self::Type>>(self, value: V) -> (String, Bson);
}

impl<T> SetAble for T
where
    T: Field,
    T::Type: Into<Bson>,
{
    fn set<V: Into<T::Type>>(self, value: V) -> (String, Bson) {
        (Self::NAME.to_string(), bson!(value.into()))
    }
}

/// SortAble
///
/// Allows a certain field to be sorted
pub trait SortAble: Field + Sized {
    /// Sort as descending
    fn desc(self) -> Document;

    /// Sort as ascending
    fn asc(self) -> Document;
}

impl<T> SortAble for T
where
    T: Field,
{
    fn asc(self) -> Document {
        let name = Self::NAME.to_string();
        doc! {
            name: 1
        }
    }

    fn desc(self) -> Document {
        let name = Self::NAME.to_string();
        doc! {
            name: -1
        }
    }
}
