//! $`a \circ b \to min(a, b)`$
use crate::algebra::{Associative, BoundedAbove, Commutative, Idempotent, Magma, Unital};
use crate::*;

#[derive(Clone, Debug)]
pub struct Minimization<S>(Infallible, PhantomData<fn() -> S>);
impl<S: BoundedAbove + Copy + PartialOrd + Debug> Magma for Minimization<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        if x > y {
            *y
        } else {
            *x
        }
    }
}
impl<S: BoundedAbove + Copy + PartialOrd + Debug> Unital for Minimization<S> {
    fn unit() -> Self::M {
        S::max_value()
    }
}
impl<S: BoundedAbove + Copy + PartialOrd + Debug> Associative for Minimization<S> {}
impl<S: BoundedAbove + Copy + PartialOrd + Debug> Commutative for Minimization<S> {}
impl<S: BoundedAbove + Copy + PartialOrd + Debug> Idempotent for Minimization<S> {}
