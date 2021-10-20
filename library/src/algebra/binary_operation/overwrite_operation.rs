//! $`a \circ b \to a`$
/// 注意 テストできてない
use crate::algebra::{Associative, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct OverwriteOperation<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Zero + Copy + Ord + Debug> Magma for OverwriteOperation<S> {
    type M = Option<S>;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        match (x, y) {
            (_, Some(y)) => Some(*y),
            (Some(x), _) => Some(*x),
            _ => None,
        }
    }
}
impl<S: Zero + Copy + Ord + Debug> Unital for OverwriteOperation<S> {
    fn unit() -> Self::M {
        None
    }
}
impl<S: Zero + Copy + Ord + Debug> Associative for OverwriteOperation<S> {}
