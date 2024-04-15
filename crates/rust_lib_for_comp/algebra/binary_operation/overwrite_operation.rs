//! # 上書き $a \circ b \to a$
//!
//! ## verify
//! [ABL E](https://atcoder.jp/contests/abl/submissions/26979080)
use crate::algebra::*;

#[codesnip::entry("overwrite_operation", include("algebra"))]
pub use overwrite_operation_mod::OverwriteOperation;
#[codesnip::entry("overwrite_operation", include("algebra"))]
mod overwrite_operation_mod {
    use super::{Associative, Debug, Default, Idempotent, Magma, Mapping, PhantomData, Unital};
    #[derive(Clone, Debug, Default)]
    pub struct OverwriteOperation<S>(PhantomData<fn() -> S>);
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
    impl<S: Clone + Debug + PartialEq> Mapping for OverwriteOperation<S> {
        type Mapping = S;
        type Domain = S;
        type Codomain = S;
        fn apply(map: &Self::Mapping, _: &Self::Domain) -> Self::Codomain {
            map.clone()
        }
    }
}
