//! $`a \circ b \to or(a, b)`$
use crate::algebra::{Associative, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct OrMonoid<S>(Infallible, PhantomData<fn() -> S>);

impl<S: Zero + Copy + BitOr<Output = S> + Ord + Debug> Magma for OrMonoid<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        *x | *y
    }
}

impl<S: Zero + Copy + BitOr<Output = S> + Ord + Debug> Associative for OrMonoid<S> {}

impl<S: Zero + Copy + BitOr<Output = S> + Ord + Debug> Unital for OrMonoid<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}
