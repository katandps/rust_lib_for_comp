//! # 加算 $a \circ b \to sum(a, b)$
//!
//! ## verify
//! [ABL E](https://atcoder.jp/contests/abl/submissions/26979080)

use algebra::*;
use prelude::*;

#[snippet(name = "addition", doc_hidden)]
#[derive(Clone, Debug, Default)]
pub struct Addition<S>(PhantomData<fn() -> S>);
#[snippet(name = "addition", doc_hidden)]
mod addition_impl {
    use super::{
        Add, Addition, Associative, Commutative, Debug, Invertible, Magma, Neg, Unital, Zero,
    };
    impl<S: Clone + Debug + Add<Output = S> + PartialEq> Magma for Addition<S> {
        type M = S;
        fn op(x: &S, y: &S) -> S {
            x.clone() + y.clone()
        }
    }
    impl<S: Clone + Debug + Add<Output = S> + PartialEq> Associative for Addition<S> {}
    impl<S: Clone + Debug + Add<Output = S> + PartialEq + Zero> Unital for Addition<S> {
        fn unit() -> S {
            S::zero()
        }
    }
    impl<S: Clone + Debug + Add<Output = S> + PartialEq> Commutative for Addition<S> {}
    impl<S: Clone + Debug + Add<Output = S> + PartialEq + Neg<Output = S>> Invertible for Addition<S> {
        fn inv(x: &S) -> S {
            x.clone().neg()
        }
    }
}
