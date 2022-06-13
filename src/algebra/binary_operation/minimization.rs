//! # 最小化 $`a \circ b \to min(a, b)`$
use crate::algebra::{Associative, BoundedAbove, Commutative, Idempotent, Magma, Unital};
use crate::prelude::*;

#[snippet(name = "minimization", doc_hidden)]
#[derive(Clone, Debug)]
pub struct Minimization<S>(Infallible, PhantomData<fn() -> S>);

#[snippet(name = "minimization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd> Magma for Minimization<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        if x <= y {
            x.clone()
        } else {
            y.clone()
        }
    }
}

#[snippet(name = "minimization", doc_hidden)]
impl<S: BoundedAbove + Debug + Clone + PartialOrd> Unital for Minimization<S> {
    fn unit() -> Self::M {
        S::max_value()
    }
}

#[snippet(name = "minimization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd> Associative for Minimization<S> {}

#[snippet(name = "minimization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd> Commutative for Minimization<S> {}

#[snippet(name = "minimization", doc_hidden)]
impl<S: Clone + Debug + PartialOrd> Idempotent for Minimization<S> {}
