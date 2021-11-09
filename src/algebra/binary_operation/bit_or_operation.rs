//! # BitOr $`a \circ b \to or(a, b)`$
use crate::algebra::{Associative, Commutative, Idempotent, Magma, Unital, Zero};
use crate::prelude::*;

pub struct BitOrOperation<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Clone + BitOr<Output = S> + PartialEq> Magma for BitOrOperation<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        x.clone() | y.clone()
    }
}
impl<S: Zero + Copy + BitOr<Output = S> + PartialEq> Unital for BitOrOperation<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}
impl<S: Clone + BitOr<Output = S> + PartialEq> Associative for BitOrOperation<S> {}
impl<S: Clone + BitOr<Output = S> + PartialEq> Commutative for BitOrOperation<S> {}
impl<S: Clone + BitOr<Output = S> + PartialEq> Idempotent for BitOrOperation<S> {}
