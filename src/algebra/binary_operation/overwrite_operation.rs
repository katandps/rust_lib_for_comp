//! # 上書き $`a \circ b \to a`$
//!
//! ## verify
//! [ABL E](https://atcoder.jp/contests/abl/submissions/26979080)
use crate::algebra::{Associative, Idempotent, Magma, Unital};
use crate::prelude::*;

#[snippet(name = "overwrite_operation", doc_hidden)]
pub struct OverwriteOperation<S>(Infallible, PhantomData<fn() -> S>);

#[snippet(name = "overwrite_operation", doc_hidden)]
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

#[snippet(name = "overwrite_operation", doc_hidden)]
impl<S: Clone + PartialEq> Unital for OverwriteOperation<S> {
    fn unit() -> Self::M {
        None
    }
}

#[snippet(name = "overwrite_operation", doc_hidden)]
impl<S: Clone + PartialEq> Associative for OverwriteOperation<S> {}

#[snippet(name = "overwrite_operation", doc_hidden)]
impl<S: Clone + PartialEq> Idempotent for OverwriteOperation<S> {}
