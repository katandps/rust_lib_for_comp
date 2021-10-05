//! $`a \circ b \to a`$
/// 注意 テストできてない
use crate::algebra::{Associative, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct Overwrite<S>(Infallible, PhantomData<fn() -> S>);

impl<S: Zero + Copy + Ord + Debug> Magma for Overwrite<S> {
    type M = Option<S>;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        match (x, y) {
            (_, Some(y)) => Some(*y),
            (Some(x), _) => Some(*x),
            _ => None,
        }
    }
}

impl<S: Zero + Copy + Ord + Debug> Associative for Overwrite<S> {}

impl<S: Zero + Copy + Ord + Debug> Unital for Overwrite<S> {
    fn unit() -> Self::M {
        None
    }
}
