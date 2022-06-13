//! # 最大化 $`a \circ b \to max(a, b)`$
use crate::algebra::{Associative, BoundedBelow, Commutative, Idempotent, Magma, Unital};
use crate::prelude::*;

#[snippet(name = "maximization", doc_hidden)]
#[derive(Debug)]
pub struct Maximization<S>(Infallible, PhantomData<fn() -> S>);

#[snippet(name = "maximization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd> Magma for Maximization<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        if x >= y {
            x.clone()
        } else {
            y.clone()
        }
    }
}

#[snippet(name = "maximization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd + BoundedBelow> Unital for Maximization<S> {
    fn unit() -> Self::M {
        S::min_value()
    }
}

#[snippet(name = "maximization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd> Associative for Maximization<S> {}

#[snippet(name = "maximization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd> Commutative for Maximization<S> {}

#[snippet(name = "maximization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd> Idempotent for Maximization<S> {}
