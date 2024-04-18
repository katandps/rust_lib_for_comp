//! # BitXor $a \circ b \to xor(a, b)$
use crate::algebra::*;

#[codesnip::entry("bit-xor-operation", include("algebra"))]
pub use bit_xor_operation_impl::BitXorOperation;
#[codesnip::entry("bit-xor-operation", include("algebra"))]
mod bit_xor_operation_impl {
    use super::{
        Associative, BitXor, Commutative, Debug, Default, Magma, PhantomData, Unital, Zero,
    };
    #[derive(Clone, Debug, Default)]
    pub struct BitXorOperation<S>(PhantomData<fn() -> S>);
    impl<S: Clone + Debug + BitXor<Output = S> + PartialEq> Magma for BitXorOperation<S> {
        type M = S;
        fn op(&mut self, x: &Self::M, y: &Self::M) -> Self::M {
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
