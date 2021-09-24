//! $`a \circ b \to min(a, b)`$
use crate::algebra::{Associative, BoundedAbove, Magma, SemiGroup, Unital};

pub use min::*;
pub mod min {
    use super::*;
    use std::convert::Infallible;
    use std::marker::PhantomData;

    #[derive(Clone, Debug)]
    pub struct Min<S>(Infallible, PhantomData<fn() -> S>);

    impl<S> SemiGroup for Min<S> where S: BoundedAbove + Copy + Ord {}

    impl<S> Magma for Min<S>
    where
        S: BoundedAbove + Copy + Ord,
    {
        type M = S;

        fn op(x: &Self::M, y: &Self::M) -> Self::M {
            std::cmp::min(*x, *y)
        }
    }

    impl<S> Associative for Min<S> where S: BoundedAbove + Copy + Ord {}

    impl<S> Unital for Min<S>
    where
        S: BoundedAbove + Copy + Ord,
    {
        fn unit() -> Self::M {
            S::max_value()
        }
    }
}
