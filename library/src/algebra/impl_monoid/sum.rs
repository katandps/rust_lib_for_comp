//! $`a \circ b \to sum(a, b)`$
use crate::algebra::{Associative, Magma, Unital, Zero};

use std::convert::Infallible;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct Sum<S>(Infallible, PhantomData<fn() -> S>);

impl<S: Zero + Copy + Add<Output = S> + Ord + Debug> Magma for Sum<S> {
    type M = S;
    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        *x + *y
    }
}

impl<S: Zero + Copy + Add<Output = S> + Ord + Debug> Associative for Sum<S> {}

impl<S: Zero + Copy + Add<Output = S> + Ord + Debug> Unital for Sum<S> {
    fn unit() -> Self::M {
        S::zero()
    }
}
