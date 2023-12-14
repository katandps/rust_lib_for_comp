//! # 加算 $a \circ b \to sum(a, b)$
//!

use crate::algebra::*;
use crate::prelude::*;

#[codesnip::entry("addition", doc_hidden)]
#[derive(Clone, Debug, Default)]
pub struct Addition<S>(PhantomData<fn() -> S>);
#[codesnip::entry("addition", doc_hidden)]
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

#[test]
fn test() {
    assert_eq!(0, Addition::unit());
    let addition = Addition::default();
    assert_eq!(15, addition.clone().pow(1i64, 15));
    assert_eq!(25600, addition.pow(100i64, 256));
    let _ = format!("{:?}", addition);
}
