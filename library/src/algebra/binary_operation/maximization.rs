//! $`a \circ b \to max(a, b)`$
use crate::algebra::{Associative, BoundedBelow, Commutative, Idempotent, Magma, Unital};

use std::convert::Infallible;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct Maximization<S>(Infallible, PhantomData<fn() -> S>);
impl<S: BoundedBelow + Copy + PartialOrd + Debug> Magma for Maximization<S> {
    type M = S;

    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        if x < y {
            *y
        } else {
            *x
        }
    }
}
impl<S: BoundedBelow + Copy + PartialOrd + Debug> Unital for Maximization<S> {
    fn unit() -> Self::M {
        S::min_value()
    }
}
impl<S: BoundedBelow + Copy + PartialOrd + Debug> Associative for Maximization<S> {}
impl<S: BoundedBelow + Copy + PartialOrd + Debug> Commutative for Maximization<S> {}
impl<S: BoundedBelow + Copy + PartialOrd + Debug> Idempotent for Maximization<S> {}
