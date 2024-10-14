#[macro_export]
macro_rules! comparison_type {
    ($name:ident, $op:expr) => {
        pub struct $name<Lhs, Rhs>(Lhs, Rhs);

        impl<Lhs, Rhs> ComparisonItem for $name<Lhs, Rhs>
        where
            Lhs: Expression,
            Rhs: Expression,
        {
            type Lhs = Lhs;
            type Rhs = Rhs;

            const OP: &'static str = $op;

            fn new(lhs: Lhs, rhs: Rhs) -> Self {
                $name(lhs, rhs)
            }

            fn lhs(&self) -> Self::Lhs {
                self.0.clone()
            }

            fn rhs(&self) -> Self::Rhs {
                self.1.clone()
            }
        }
    };
}

#[macro_export]
macro_rules! comparison {
    () => {
        pub trait Comparisons: Comparable + Sized {
            comparison!(fn eq Eq, fn ne NotEq, fn gt Gt, fn gte GtEq, fn lt Lt, fn lte LtEq, fn in_ In, fn not_in NotIn);
        }
    };

    (fn $name:ident $op:ident) => {
        fn $name<T>(self, other: T) -> ComparisonBackoff<$op<impl Expression, impl Expression>>
        where
            T: AsExpression,
        {
            ComparisonBackoff($op::new(self.expression(), other.as_expression()))
        }
    };

    (fn $name:ident $op:ident, $($rest:tt)*) => {
        fn $name<T>(self, other: T) -> ComparisonBackoff<$op<impl Expression, impl Expression>>
        where
            T: AsExpression,
        {
            ComparisonBackoff($op::new(self.expression(), other.as_expression()))
        }
        comparison!($($rest)*);
    };

    ($($rest:tt)*) => {
        comparison!($($rest)*);
    };
}
