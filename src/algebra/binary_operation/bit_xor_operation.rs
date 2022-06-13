//! # BitXor $`a \circ b \to xor(a, b)`$
use crate::algebra::{Associative, Commutative, Magma, Unital, Zero};
use crate::prelude::*;

#[snippet(name = "bit-xor-operation", doc_hidden)]
pub struct BitXorOperation<S>(Infallible, PhantomData<fn() -> S>);

#[snippet(name = "bit-xor-operation", doc_hidden)]
impl<S: Clone + Debug + BitXor<Output = S> + PartialEq> Magma for BitXorOperation<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        x.clone() ^ y.clone()
    }
}

#[snippet(name = "bit-xor-operation", doc_hidden)]
impl<S: Clone + Debug + BitXor<Output = S> + PartialEq + Zero> Unital for BitXorOperation<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}

#[snippet(name = "bit-xor-operation", doc_hidden)]
impl<S: Clone + Debug + BitXor<Output = S> + PartialEq> Associative for BitXorOperation<S> {}

#[snippet(name = "bit-xor-operation", doc_hidden)]
impl<S: Clone + Debug + BitXor<Output = S> + PartialEq> Commutative for BitXorOperation<S> {}
