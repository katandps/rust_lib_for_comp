//! $`a \circ b \to xor(a, b)`$
use crate::algebra::{Associative, Commutative, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct BitXorOperation<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Zero + Copy + BitXor<Output = S> + Ord + Debug> Magma for BitXorOperation<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        *x ^ *y
    }
}
impl<S: Zero + Copy + BitXor<Output = S> + Ord + Debug> Unital for BitXorOperation<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}
impl<S: Zero + Copy + BitXor<Output = S> + Ord + Debug> Associative for BitXorOperation<S> {}
impl<S: Zero + Copy + BitXor<Output = S> + Ord + Debug> Commutative for BitXorOperation<S> {}
