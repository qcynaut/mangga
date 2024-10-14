use super::logicals::IntoLogical;
use crate::{
    comparison,
    comparison_type,
    traits::{AsExpression, Expression},
};

/// ComparisonItem
pub trait ComparisonItem: Sized {
    type Lhs: Expression;
    type Rhs: Expression;

    /// Operator
    const OP: &'static str;

    /// Creates a new comparison
    fn new(lhs: Self::Lhs, rhs: Self::Rhs) -> Self;

    /// Returns the left hand side
    fn lhs(&self) -> Self::Lhs;

    /// Returns the right hand side
    fn rhs(&self) -> Self::Rhs;

    /// Returns the expression
    fn expression(self) -> impl Expression {
        let lhs = self
            .lhs()
            .build()
            .as_str()
            .map(|s| s.to_owned())
            .unwrap_or_default();
        let rhs = self.rhs().build();
        let op = Self::OP;
        bson::bson!({lhs: {op: rhs}})
    }
}

/// Comparable
pub trait Comparable {
    fn expression(self) -> impl Expression;
}

/// ComparisonBackoff
pub struct ComparisonBackoff<T: ComparisonItem>(pub(crate) T);

comparison_type!(Eq, "$eq");
comparison_type!(NotEq, "$ne");
comparison_type!(Gt, "$gt");
comparison_type!(GtEq, "$gte");
comparison_type!(Lt, "$lt");
comparison_type!(LtEq, "$lte");
comparison_type!(In, "$in");
comparison_type!(NotIn, "$nin");

comparison!();

impl<T: Comparable> Comparisons for T {}
impl<T: Expression> Comparable for T {
    fn expression(self) -> impl Expression {
        self
    }
}
impl<T: ComparisonItem> Comparable for ComparisonBackoff<T>
where
    T::Lhs: Expression,
{
    fn expression(self) -> impl Expression {
        self.0.lhs()
    }
}
impl<T: ComparisonItem> IntoLogical for ComparisonBackoff<T> {
    fn into_logical(self) -> impl Expression {
        self.0.expression()
    }
}
