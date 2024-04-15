//! # 加算 $a \circ b \to sum(a, b)$
//!

use crate::algebra::*;

#[codesnip::entry("addition", include("algebra"))]
#[derive(Clone, Debug, Default)]
pub struct Addition<Lhs, Rhs = Lhs>(PhantomData<fn() -> (Lhs, Rhs)>);
#[codesnip::entry("addition", include("algebra"))]
mod addition_impl {
    use super::{
        Add, Addition, Associative, Commutative, Debug, Invertible, Magma, Mapping, Neg, Unital,
        Zero,
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
    impl<S: Clone + Debug + Add<T, Output = T> + PartialEq, T: Clone + Debug> Mapping
        for Addition<S, T>
    {
        type Mapping = S;
        type Domain = T;
        type Codomain = T;
        fn apply(map: &Self::Mapping, a: &Self::Domain) -> Self::Codomain {
            map.clone() + a.clone()
        }
    }
}

#[test]
fn test() {
    assert_eq!(0, Addition::<i64>::unit());
    let addition = Addition::default();
    assert_eq!(15, addition.clone().pow(1i64, 15));
    assert_eq!(25600, addition.pow(100i64, 256));
    let _ = format!("{:?}", addition);
}
