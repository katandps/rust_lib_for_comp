//! # 最大化 $`a \circ b \to max(a, b)`$
use crate::algebra::{Associative, BoundedBelow, Commutative, Idempotent, Magma, Unital};
use crate::prelude::*;

pub struct Maximization<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Clone + PartialOrd> Magma for Maximization<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        if x >= y {
            x.clone()
        } else {
            y.clone()
        }
    }
}
impl<S: Clone + PartialOrd + BoundedBelow> Unital for Maximization<S> {
    fn unit() -> Self::M {
        S::min_value()
    }
}
impl<S: Clone + PartialOrd> Associative for Maximization<S> {}
impl<S: Clone + PartialOrd> Commutative for Maximization<S> {}
impl<S: Clone + PartialOrd> Idempotent for Maximization<S> {}
