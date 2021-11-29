//! # 加算 $`a \circ b \to sum(a, b)`$
//!
//! ## verify
//! [ABL E](https://atcoder.jp/contests/abl/submissions/26979080)

use crate::algebra::{Associative, Commutative, Invertible, Magma, Unital, Zero};
use crate::prelude::*;

#[snippet(name = "addition", doc_hidden)]
pub struct Addition<S>(Infallible, PhantomData<fn() -> S>);

#[snippet(name = "addition", doc_hidden)]
impl<S: Clone + Add<Output = S> + PartialEq> Magma for Addition<S> {
    type M = S;
    fn op(x: &S, y: &S) -> S {
        x.clone() + y.clone()
    }
}

#[snippet(name = "addition", doc_hidden)]
impl<S: Clone + Add<Output = S> + PartialEq> Associative for Addition<S> {}

#[snippet(name = "addition", doc_hidden)]
impl<S: Clone + Add<Output = S> + PartialEq + Zero> Unital for Addition<S> {
    fn unit() -> S {
        S::zero()
    }
}

#[snippet(name = "addition", doc_hidden)]
impl<S: Clone + Add<Output = S> + PartialEq> Commutative for Addition<S> {}

#[snippet(name = "addition", doc_hidden)]
impl<S: Clone + Add<Output = S> + PartialEq + Neg<Output = S>> Invertible for Addition<S> {
    fn inv(x: &S) -> S {
        x.clone().neg()
    }
}
