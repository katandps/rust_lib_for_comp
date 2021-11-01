//! # 最小化 $`a \circ b \to min(a, b)`$
use crate::algebra::{Associative, BoundedAbove, Commutative, Idempotent, Magma, Unital};
use crate::*;

pub struct Minimization<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Clone + PartialOrd> Magma for Minimization<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        if x <= y {
            y.clone()
        } else {
            x.clone()
        }
    }
}
impl<S: BoundedAbove + Clone + PartialOrd> Unital for Minimization<S> {
    fn unit() -> Self::M {
        S::max_value()
    }
}
impl<S: Clone + PartialOrd> Associative for Minimization<S> {}
impl<S: Clone + PartialOrd> Commutative for Minimization<S> {}
impl<S: Clone + PartialOrd> Idempotent for Minimization<S> {}
