//! # 上書き $a \circ b \to a$
//!
//! ## verify
//! [ABL E](https://atcoder.jp/contests/abl/submissions/26979080)
use crate::prelude::*;

#[snippet(name = "overwrite_operation", doc_hidden)]
#[derive(Clone, Debug)]
pub struct OverwriteOperation<S>(Infallible, PhantomData<fn() -> S>);
#[snippet(name = "overwrite_operation", doc_hidden)]
mod overwrite_operation_mod {
    use super::{Associative, Debug, Idempotent, Magma, OverwriteOperation, Unital};
    impl<S: Clone + Debug + PartialEq> Magma for OverwriteOperation<S> {
        type M = Option<S>;
        fn op(x: &Self::M, y: &Self::M) -> Self::M {
            match (x, y) {
                (_, Some(y)) => Some(y.clone()),
                (Some(x), _) => Some(x.clone()),
                _ => None,
            }
        }
    }
    impl<S: Clone + Debug + PartialEq> Unital for OverwriteOperation<S> {
        fn unit() -> Self::M {
            None
        }
    }
    impl<S: Clone + Debug + PartialEq> Associative for OverwriteOperation<S> {}
    impl<S: Clone + Debug + PartialEq> Idempotent for OverwriteOperation<S> {}
}
