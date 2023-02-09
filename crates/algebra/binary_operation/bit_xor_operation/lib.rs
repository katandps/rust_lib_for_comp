//! # BitXor $a \circ b \to xor(a, b)$
use algebra::*;
use prelude::*;

#[snippet(name = "bit-xor-operation", doc_hidden)]
pub use bit_xor_operation_impl::BitXorOperation;
#[snippet(name = "bit-xor-operation", doc_hidden)]
mod bit_xor_operation_impl {
    use super::{
        Associative, BitXor, Commutative, Debug, Default, Magma, PhantomData, Unital, Zero,
    };
    #[derive(Clone, Debug, Default)]
    pub struct BitXorOperation<S>(PhantomData<fn() -> S>);
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
