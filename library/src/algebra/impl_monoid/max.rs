//! $`a \circ b \to max(a, b)`$
use crate::algebra::{Associative, BoundedBelow, Magma, SemiGroup, Unital};

pub use max::*;
pub mod max {
    use super::*;
    use std::convert::Infallible;
    use std::marker::PhantomData;

    #[derive(Clone, Debug)]
    pub struct Max<S>(Infallible, PhantomData<fn() -> S>);

    impl<S> SemiGroup for Max<S> where S: BoundedBelow + Copy + Ord {}

    impl<S> Magma for Max<S>
    where
        S: BoundedBelow + Copy + Ord,
    {
        type M = S;

        fn op(x: &Self::M, y: &Self::M) -> Self::M {
            std::cmp::max(*x, *y)
        }
    }

    impl<S> Associative for Max<S> where S: BoundedBelow + Copy + Ord {}

    impl<S> Unital for Max<S>
    where
        S: BoundedBelow + Copy + Ord,
    {
        fn unit() -> Self::M {
            S::min_value()
        }
    }
}
