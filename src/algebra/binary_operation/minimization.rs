//! # 最小化 $a \circ b \to min(a, b)$
//! $a > b$ のとき b
use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Minimization<S>(PhantomData<fn() -> S>);
mod minimization_impl {
    use super::{
        Associative, BoundedAbove, Commutative, Debug, Idempotent, Magma, Minimization, Unital,
    };
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
    impl<S: BoundedAbove + Debug + Clone + PartialOrd> Unital for Minimization<S> {
        fn unit() -> Self::M {
            S::max_value()
        }
    }
    impl<S: Clone + Debug + PartialOrd> Associative for Minimization<S> {}
    impl<S: Clone + Debug + PartialOrd> Commutative for Minimization<S> {}
    impl<S: Clone + Debug + PartialOrd> Idempotent for Minimization<S> {}
}
