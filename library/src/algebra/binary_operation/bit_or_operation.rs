//! $`a \circ b \to or(a, b)`$
use crate::algebra::{Associative, Commutative, Idempotent, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct BitOrOperation<S>(Infallible, PhantomData<fn() -> S>);

impl<S: Zero + Copy + BitOr<Output = S> + Ord + Debug> Magma for BitOrOperation<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        *x | *y
    }
}
impl<S: Zero + Copy + BitOr<Output = S> + Ord + Debug> Unital for BitOrOperation<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}
impl<S: Zero + Copy + BitOr<Output = S> + Ord + Debug> Associative for BitOrOperation<S> {}
impl<S: Zero + Copy + BitOr<Output = S> + Ord + Debug> Commutative for BitOrOperation<S> {}
impl<S: Zero + Copy + BitOr<Output = S> + Ord + Debug> Idempotent for BitOrOperation<S> {}
