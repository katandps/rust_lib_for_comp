//! # BitXor $`a \circ b \to xor(a, b)`$
use crate::algebra::{Associative, Commutative, Magma, Unital, Zero};
use crate::prelude::*;

pub struct BitXorOperation<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Clone + BitXor<Output = S> + PartialEq> Magma for BitXorOperation<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        x.clone() ^ y.clone()
    }
}
impl<S: Clone + BitXor<Output = S> + PartialEq + Zero> Unital for BitXorOperation<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}
impl<S: Clone + BitXor<Output = S> + PartialEq> Associative for BitXorOperation<S> {}
impl<S: Clone + BitXor<Output = S> + PartialEq> Commutative for BitXorOperation<S> {}
