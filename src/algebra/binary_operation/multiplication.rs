//! # 乗算 $`a \circ b \to prod(a, b)`$
use crate::algebra::{Associative, Commutative, Invertible, Magma, One, Unital};
use crate::prelude::*;

#[snippet(name = "multiplication", doc_hidden)]
pub struct Multiplication<S>(Infallible, PhantomData<fn() -> S>);

#[snippet(name = "multiplication", doc_hidden)]
impl<S: Clone + Debug + Mul<Output = S> + PartialEq> Magma for Multiplication<S> {
    type M = S;
    fn op(x: &S, y: &S) -> S {
        x.clone() * y.clone()
    }
}

#[snippet(name = "multiplication", doc_hidden)]
impl<S: Clone + Debug + Mul<Output = S> + PartialEq> Associative for Multiplication<S> {}

#[snippet(name = "multiplication", doc_hidden)]
impl<S: Clone + Debug + Mul<Output = S> + PartialEq + One> Unital for Multiplication<S> {
    fn unit() -> S {
        S::one()
    }
}

#[snippet(name = "multiplication", doc_hidden)]
impl<S: Clone + Debug + Mul<Output = S> + PartialEq> Commutative for Multiplication<S> {}

#[snippet(name = "multiplication", doc_hidden)]
impl<S: Clone + Debug + Mul<Output = S> + PartialEq + One + Div<Output = S>> Invertible
    for Multiplication<S>
{
    fn inv(x: &S) -> S {
        Self::unit() / x.clone()
    }
}
