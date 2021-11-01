//! # 加算 $`a \circ b \to sum(a, b)`$
//!
//! ## verify
//! [ABL E](https://atcoder.jp/contests/abl/submissions/26979080)

use crate::algebra::{Associative, Commutative, Invertible, Magma, Unital, Zero};
use crate::*;

pub struct Addition<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Clone + Add<Output = S> + PartialEq> Magma for Addition<S> {
    type M = S;
    fn op(x: &S, y: &S) -> S {
        x.clone() + y.clone()
    }
}
impl<S: Clone + Add<Output = S> + PartialEq> Associative for Addition<S> {}
impl<S: Clone + Add<Output = S> + PartialEq + Zero> Unital for Addition<S> {
    fn unit() -> S {
        S::zero()
    }
}
impl<S: Clone + Add<Output = S> + PartialEq> Commutative for Addition<S> {}
impl<S: Clone + Add<Output = S> + PartialEq + Neg<Output = S>> Invertible for Addition<S> {
    fn inv(x: &S) -> S {
        x.clone().neg()
    }
}

/// 区間和を求めるセグメント木に載せる値
/// ### algo
/// 例えば、[0, 4)の区間に3を足した時、 合計の値は3に区間の幅をかけた12増える
/// 区間の長さを持たせることで計算できるようになる
#[derive(Clone, PartialEq, Ord, PartialOrd, Eq)]
pub struct Segment<M: Clone + PartialEq> {
    pub value: M,
    size: i64,
}
impl<M: Clone + PartialEq + Display> Debug for Segment<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v: {}, size: {}", self.value, self.size)
    }
}
impl<M: Clone + PartialEq + Add<Output = M> + Zero> Add for Segment<M> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (value, size) = (self.value + rhs.value, self.size + rhs.size);
        Self { value, size }
    }
}
impl<M: Clone + PartialEq + Zero> Zero for Segment<M> {
    fn zero() -> Self {
        let (value, size) = (M::zero(), 1);
        Self { value, size }
    }
}
impl<M: Clone + PartialEq + Add<Output = M>> Add<M> for Segment<M> {
    type Output = Self;
    fn add(self, rhs: M) -> Self {
        let (value, size) = (self.value + rhs, self.size);
        Self { value, size }
    }
}
impl<M: Clone + PartialEq + Mul<Output = M>> Mul<M> for Segment<M> {
    type Output = Self;
    fn mul(self, rhs: M) -> Self {
        let (value, size) = (self.value * rhs, self.size);
        Self { value, size }
    }
}
