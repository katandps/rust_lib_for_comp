//! $`a \circ b \to xor(a, b)`$
use crate::algebra::{Associative, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct Xor<S>(Infallible, PhantomData<fn() -> S>);

impl<S: Zero + Copy + BitXor<Output = S> + Ord + Debug> Magma for Xor<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        *x ^ *y
    }
}

impl<S: Zero + Copy + BitXor<Output = S> + Ord + Debug> Associative for Xor<S> {}

impl<S: Zero + Copy + BitXor<Output = S> + Ord + Debug> Unital for Xor<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}
