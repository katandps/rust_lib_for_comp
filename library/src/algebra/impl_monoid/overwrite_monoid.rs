//! $`a \circ b \to a`$
/// 注意 テストできてない
use crate::algebra::{Associative, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct OverwriteMonoid<S>(Infallible, PhantomData<fn() -> S>);

impl<S: Zero + Copy + Ord + Debug> Magma for OverwriteMonoid<S> {
    type M = Option<S>;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        match (x, y) {
            (_, Some(y)) => Some(*y),
            (Some(x), _) => Some(*x),
            _ => None,
        }
    }
}

impl<S: Zero + Copy + Ord + Debug> Associative for OverwriteMonoid<S> {}

impl<S: Zero + Copy + Ord + Debug> Unital for OverwriteMonoid<S> {
    fn unit() -> Self::M {
        None
    }
}
