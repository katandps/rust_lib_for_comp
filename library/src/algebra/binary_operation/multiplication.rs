//! $`a \circ b \to prod(a, b)`$
use crate::algebra::{Associative, Commutative, Invertible, Magma, One, Unital};
use crate::*;
#[derive(Clone, Debug)]
pub struct Multiplication<S>(Infallible, PhantomData<fn() -> S>);
impl<S: One + Copy + Mul<Output = S> + Ord + Debug> Magma for Multiplication<S> {
    type M = S;
    fn op(x: &S, y: &S) -> S {
        *x * *y
    }
}
impl<S: One + Copy + Mul<Output = S> + Ord + Debug> Associative for Multiplication<S> {}
impl<S: One + Copy + Mul<Output = S> + Ord + Debug> Unital for Multiplication<S> {
    fn unit() -> S {
        S::one()
    }
}
impl<S: One + Copy + Mul<Output = S> + Ord + Debug> Commutative for Multiplication<S> {}
impl<S: One + Copy + Mul<Output = S> + Ord + Debug + Div<Output = S>> Invertible
    for Multiplication<S>
{
    fn inv(x: &S) -> S {
        Self::unit() / *x
    }
}
