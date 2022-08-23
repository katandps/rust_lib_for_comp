//! # 最大化 $a \circ b \to max(a, b)$
//! $a < b$ のとき b
use crate::prelude::*;

#[snippet(name = "maximization", doc_hidden)]
#[derive(Clone, Debug)]
pub struct Maximization<S>(Infallible, PhantomData<fn() -> S>);
#[snippet(name = "maximization", doc_hidden)]
mod maximization_impl {
    use super::{
        Associative, BoundedBelow, Commutative, Debug, Idempotent, Magma, Maximization, Unital,
    };
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
    impl<S: Clone + Debug + PartialOrd + BoundedBelow> Unital for Maximization<S> {
        fn unit() -> Self::M {
            S::min_value()
        }
    }
    impl<S: Clone + Debug + PartialOrd> Associative for Maximization<S> {}
    impl<S: Clone + Debug + PartialOrd> Commutative for Maximization<S> {}
    impl<S: Clone + Debug + PartialOrd> Idempotent for Maximization<S> {}
}
