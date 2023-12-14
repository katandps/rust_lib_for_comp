//! # 区間
//! ### algo
//! 例えば、[0, 4)の区間に3を足した時、 合計の値は3に区間の幅をかけた12増える
//! 区間の長さを持たせることで区間和などを計算できるようになる

use algebra::Zero;
use prelude::*;

#[codesnip::entry("section", doc_hidden)]
pub use section_impl::Section;
#[codesnip::entry("section", doc_hidden)]
mod section_impl {
    use super::{Add, Debug, Display, Formatter, Mul, Zero};

    #[derive(Clone, PartialEq, Ord, PartialOrd, Eq)]
    pub struct Section<M: Clone + PartialEq> {
        pub value: M,
        size: i64,
    }
    impl<M: Clone + PartialEq> Section<M> {
        pub fn new(value: M, size: i64) -> Self {
            Self { value, size }
        }
    }

    impl<M: Clone + PartialEq + Display> Debug for Section<M> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "v: {}, size: {}", self.value, self.size)
        }
    }
    impl<M: Clone + PartialEq + Add<Output = M> + Zero> Add for Section<M> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            let (value, size) = (self.value + rhs.value, self.size + rhs.size);
            Self { value, size }
        }
    }
    impl<M: Clone + PartialEq + Zero> Zero for Section<M> {
        fn zero() -> Self {
            let (value, size) = (M::zero(), 0);
            Self { value, size }
        }
    }
    impl<M: Clone + PartialEq + Add<Output = M>> Add<M> for Section<M> {
        type Output = Self;
        fn add(self, rhs: M) -> Self {
            let (value, size) = (self.value + rhs, self.size);
            Self { value, size }
        }
    }
    impl<M: Clone + PartialEq + Mul<Output = M>> Mul<M> for Section<M> {
        type Output = Self;
        fn mul(self, rhs: M) -> Self {
            let (value, size) = (self.value * rhs, self.size);
            Self { value, size }
        }
    }
}
