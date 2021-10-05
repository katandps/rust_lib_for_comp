//! $`a \circ b \to min(a, b)`$
use crate::algebra::{Associative, BoundedAbove, Magma, Unital};

use std::convert::Infallible;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Min<S>(Infallible, PhantomData<fn() -> S>);

impl<S> Magma for Min<S>
where
    S: BoundedAbove + Copy + Ord + Debug,
{
    type M = S;

    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        std::cmp::min(*x, *y)
    }
}

impl<S> Associative for Min<S> {}

impl<S> Unital for Min<S>
where
    S: BoundedAbove + Copy + Ord + Debug,
{
    fn unit() -> Self::M {
        S::max_value()
    }
}
