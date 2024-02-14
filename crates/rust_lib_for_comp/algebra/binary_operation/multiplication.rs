//! # 乗算 $a \circ b \to prod(a, b)$
use crate::algebra::*;

#[codesnip::entry("multiplication", include("algebra"))]
pub use multiplication_impl::Multiplication;
#[codesnip::entry("multiplication", include("algebra"))]
mod multiplication_impl {
    use super::{
        Associative, Commutative, Debug, Default, Div, Invertible, Magma, Mul, One, PhantomData,
        Unital,
    };
    #[derive(Clone, Debug, Default)]
    pub struct Multiplication<S>(PhantomData<fn() -> S>);
    impl<S: Clone + Debug + Mul<Output = S> + PartialEq> Magma for Multiplication<S> {
        type M = S;
        fn op(x: &S, y: &S) -> S {
            x.clone() * y.clone()
        }
    }
    impl<S: Clone + Debug + Mul<Output = S> + PartialEq> Associative for Multiplication<S> {}
    impl<S: Clone + Debug + Mul<Output = S> + PartialEq + One> Unital for Multiplication<S> {
        fn unit() -> S {
            S::one()
        }
    }
    impl<S: Clone + Debug + Mul<Output = S> + PartialEq> Commutative for Multiplication<S> {}
    impl<S: Clone + Debug + Mul<Output = S> + PartialEq + One + Div<Output = S>> Invertible
        for Multiplication<S>
    {
        fn inv(x: &S) -> S {
            Self::unit() / x.clone()
        }
    }
}
