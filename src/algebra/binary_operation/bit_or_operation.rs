//! # BitOr $`a \circ b \to or(a, b)`$
use crate::algebra::{Associative, Commutative, Idempotent, Magma, Unital, Zero};
use crate::prelude::*;

#[snippet(name = "bit-or-operation", doc_hidden)]
pub struct BitOrOperation<S>(Infallible, PhantomData<fn() -> S>);

#[snippet(name = "bit-or-operation", doc_hidden)]
impl<S: Clone + Debug + BitOr<Output = S> + PartialEq> Magma for BitOrOperation<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        x.clone() | y.clone()
    }
}

#[snippet(name = "bit-or-operation", doc_hidden)]
impl<S: Clone + Debug + Zero + Copy + BitOr<Output = S> + PartialEq> Unital for BitOrOperation<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}

#[snippet(name = "bit-or-operation", doc_hidden)]
impl<S: Clone + Debug + BitOr<Output = S> + PartialEq> Associative for BitOrOperation<S> {}

#[snippet(name = "bit-or-operation", doc_hidden)]
impl<S: Clone + Debug + BitOr<Output = S> + PartialEq> Commutative for BitOrOperation<S> {}

#[snippet(name = "bit-or-operation", doc_hidden)]
impl<S: Clone + Debug + BitOr<Output = S> + PartialEq> Idempotent for BitOrOperation<S> {}
