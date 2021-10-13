//! $`a \circ b \to max(a, b)`$
use crate::algebra::{Associative, BoundedBelow, Magma, Unital};

use std::convert::Infallible;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct MaxMonoid<S>(Infallible, PhantomData<fn() -> S>);

impl<S> Magma for MaxMonoid<S>
where
    S: BoundedBelow + Copy + Ord + Debug,
{
    type M = S;

    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        std::cmp::max(*x, *y)
    }
}

impl<S> Associative for MaxMonoid<S> where S: BoundedBelow + Copy + Ord + Debug {}

impl<S> Unital for MaxMonoid<S>
where
    S: BoundedBelow + Copy + Ord + Debug,
{
    fn unit() -> Self::M {
        S::min_value()
    }
}
