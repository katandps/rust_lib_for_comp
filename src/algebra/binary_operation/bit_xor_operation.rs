//! # BitXor $`a \circ b \to xor(a, b)`$
use crate::prelude::*;

#[snippet(name = "bit-xor-operation", doc_hidden)]
#[derive(Clone, Debug)]
pub struct BitXorOperation<S>(Infallible, PhantomData<fn() -> S>);
#[snippet(name = "bit-xor-operation", doc_hidden)]
mod bit_xor_operation_impl {
    use super::{Associative, BitXor, BitXorOperation, Commutative, Debug, Magma, Unital, Zero};
    impl<S: Clone + Debug + BitXor<Output = S> + PartialEq> Magma for BitXorOperation<S> {
        type M = S;
        fn op(x: &Self::M, y: &Self::M) -> Self::M {
            x.clone() ^ y.clone()
        }
    }
    impl<S: Clone + Debug + BitXor<Output = S> + PartialEq + Zero> Unital for BitXorOperation<S> {
        fn unit() -> Self::M {
            S::zero()
        }
    }
    impl<S: Clone + Debug + BitXor<Output = S> + PartialEq> Associative for BitXorOperation<S> {}
    impl<S: Clone + Debug + BitXor<Output = S> + PartialEq> Commutative for BitXorOperation<S> {}
}
