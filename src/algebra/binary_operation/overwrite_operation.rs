//! # 上書き $`a \circ b \to a`$
//!
//! ## verify
//! [ABL E](https://atcoder.jp/contests/abl/submissions/26979080)
use crate::algebra::{Associative, Idempotent, Magma, Unital};
use crate::prelude::*;

pub struct OverwriteOperation<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Clone + PartialEq> Magma for OverwriteOperation<S> {
    type M = Option<S>;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        match (x, y) {
            (_, Some(y)) => Some(y.clone()),
            (Some(x), _) => Some(x.clone()),
            _ => None,
        }
    }
}
impl<S: Clone + PartialEq> Unital for OverwriteOperation<S> {
    fn unit() -> Self::M {
        None
    }
}
impl<S: Clone + PartialEq> Associative for OverwriteOperation<S> {}
impl<S: Clone + PartialEq> Idempotent for OverwriteOperation<S> {}
