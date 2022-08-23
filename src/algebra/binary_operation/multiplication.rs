//! # 乗算 $a \circ b \to prod(a, b)$
use crate::prelude::*;

#[snippet(name = "multiplication", doc_hidden)]
#[derive(Clone, Debug)]
pub struct Multiplication<S>(Infallible, PhantomData<fn() -> S>);
#[snippet(name = "multiplication", doc_hidden)]
mod multiplication_impl {
    use super::{
        Associative, Commutative, Debug, Div, Invertible, Magma, Mul, Multiplication, One, Unital,
    };
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
