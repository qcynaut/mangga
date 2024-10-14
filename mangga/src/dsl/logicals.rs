use super::filter::IntoFilter;
use crate::traits::Expression;

/// Marker trait to indicate that the expression is a logical
pub trait IntoLogical {
    fn into_logical(self) -> impl Expression;
}

/// And
pub struct And(Vec<bson::Bson>);

/// Or
pub struct Or(Vec<bson::Bson>);

/// Logicals
pub trait Logicals {
    /// $and
    fn and<O: Logicals>(self, other: O) -> And;

    /// $or
    fn or<O: Logicals>(self, other: O) -> Or;

    /// Build the expression
    fn logical(self) -> impl Expression;
}

impl<T: IntoLogical> Logicals for T {
    fn and<O: Logicals>(self, other: O) -> And {
        And(vec![self.into_logical().build(), other.logical().build()])
    }

    fn or<O: Logicals>(self, other: O) -> Or {
        Or(vec![self.into_logical().build(), other.logical().build()])
    }

    fn logical(self) -> impl Expression {
        self.into_logical()
    }
}

impl Logicals for And {
    fn and<O: Logicals>(self, other: O) -> And {
        let mut v = self.0;
        v.push(other.logical().build());
        And(v)
    }

    fn or<O: Logicals>(self, other: O) -> Or {
        Or(vec![self.logical().build(), other.logical().build()])
    }

    fn logical(self) -> impl Expression {
        bson::bson!({ "$and": self.0 })
    }
}

impl Logicals for Or {
    fn and<O: Logicals>(self, other: O) -> And {
        And(vec![self.logical().build(), other.logical().build()])
    }

    fn or<O: Logicals>(self, other: O) -> Or {
        let mut v = self.0;
        v.push(other.logical().build());
        Or(v)
    }

    fn logical(self) -> impl Expression {
        bson::bson!({ "$or": self.0 })
    }
}

impl<T: Logicals> IntoFilter for T {
    fn into_filter(self) -> impl Expression {
        self.logical()
    }
}
