//! # BitOr $a \circ b \to or(a, b)$
use crate::prelude::*;

#[snippet(name = "bit-or-operation", doc_hidden)]
#[derive(Clone, Debug, Default)]
pub struct BitOrOperation<S>(PhantomData<fn() -> S>);
#[snippet(name = "bit-or-operation", doc_hidden)]
mod bit_or_opration_impl {
    use super::{
        Associative, BitOr, BitOrOperation, Commutative, Debug, Idempotent, Magma, Unital, Zero,
    };
    impl<S: Clone + Debug + BitOr<Output = S> + PartialEq> Magma for BitOrOperation<S> {
        type M = S;
        fn op(x: &Self::M, y: &Self::M) -> Self::M {
            x.clone() | y.clone()
        }
    }
    impl<S: Clone + Debug + Zero + Copy + BitOr<Output = S> + PartialEq> Unital for BitOrOperation<S> {
        fn unit() -> Self::M {
            S::zero()
        }
    }
    impl<S: Clone + Debug + BitOr<Output = S> + PartialEq> Associative for BitOrOperation<S> {}
    impl<S: Clone + Debug + BitOr<Output = S> + PartialEq> Commutative for BitOrOperation<S> {}
    impl<S: Clone + Debug + BitOr<Output = S> + PartialEq> Idempotent for BitOrOperation<S> {}
}
