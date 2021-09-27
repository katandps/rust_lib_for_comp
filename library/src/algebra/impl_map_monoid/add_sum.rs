//! 作用付きモノイド: 加算＋区間和
use crate::algebra::impl_monoid::sum::Sum;
use crate::algebra::{Associative, Magma, MapMonoid, Monoid, Unital, Zero};

pub use add_sum::*;
pub mod add_sum {
    use super::*;
    use std::fmt::{Debug, Formatter};
    use std::ops::Add;

    #[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
    pub struct S {
        pub value: i64,
        size: i64,
    }

    impl Debug for S {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "v: {}, size: {}", self.value, self.size)
        }
    }

    impl Add<S> for S {
        type Output = S;

        fn add(self, rhs: S) -> Self::Output {
            Self {
                value: self.value + rhs.value,
                size: self.size + rhs.size,
            }
        }
    }

    impl Add<i64> for &S {
        type Output = S;
        fn add(self, rhs: i64) -> Self::Output {
            S {
                value: self.value + rhs * self.size,
                size: self.size,
            }
        }
    }

    impl Zero for S {
        fn zero() -> Self {
            S { value: 0, size: 1 }
        }
    }

    impl Magma for S {
        type M = S;

        fn op(x: &Self::M, y: &Self::M) -> Self::M {
            *x + *y
        }
    }

    impl Associative for S {}

    impl Unital for S {
        fn unit() -> Self::M {
            Self::zero()
        }
    }

    #[derive(Debug)]
    pub struct AddSum;
    impl MapMonoid for AddSum {
        type Mono = Sum<S>;
        type Func = i64;

        fn apply(f: &Self::Func, value: &<Self::Mono as Monoid>::M) -> <Self::Mono as Monoid>::M {
            value + *f
        }

        fn identity_map() -> Self::Func {
            0
        }

        fn compose(f: &Self::Func, g: &Self::Func) -> Self::Func {
            f + g
        }
    }
}
