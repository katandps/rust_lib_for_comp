//! $`a \circ b \to sum(a, b)`$
use crate::algebra::{Associative, Commutative, Invertible, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct Addition<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Zero + Copy + Add<Output = S> + Ord + Debug> Magma for Addition<S> {
    type M = S;
    fn op(x: &S, y: &S) -> S {
        *x + *y
    }
}
impl<S: Zero + Copy + Add<Output = S> + Ord + Debug> Associative for Addition<S> {}
impl<S: Zero + Copy + Add<Output = S> + Ord + Debug> Unital for Addition<S> {
    fn unit() -> S {
        S::zero()
    }
}
impl<S: Zero + Copy + Add<Output = S> + Ord + Debug> Commutative for Addition<S> {}
impl<S: Zero + Copy + Add<Output = S> + Ord + Debug + Neg<Output = S>> Invertible for Addition<S> {
    fn inv(x: &S) -> S {
        (*x).neg()
    }
}

/// 区間和を求めるセグメント木に載せる値
/// ### algo
/// 例えば、[0, 4)の区間に3を足した時、 合計の値は3に区間の幅をかけた12増える
/// 区間の長さを持たせることで計算できるようになる
#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub struct Segment {
    pub value: i64,
    size: i64,
}

impl Debug for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v: {}, size: {}", self.value, self.size)
    }
}

impl Add<Segment> for Segment {
    type Output = Segment;

    fn add(self, rhs: Segment) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            size: self.size + rhs.size,
        }
    }
}

impl Add<i64> for &Segment {
    type Output = Segment;
    fn add(self, rhs: i64) -> Self::Output {
        Segment {
            value: self.value + rhs * self.size,
            size: self.size,
        }
    }
}

impl Zero for Segment {
    fn zero() -> Self {
        Segment { value: 0, size: 1 }
    }
}

impl Magma for Segment {
    type M = Segment;

    fn op(x: &Self::M, y: &Self::M) -> Self::M {
        *x + *y
    }
}

impl Associative for Segment {}

impl Unital for Segment {
    fn unit() -> Self::M {
        Self::zero()
    }
}
