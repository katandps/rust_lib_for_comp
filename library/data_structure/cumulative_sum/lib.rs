//! # 累積和
//!
//! イテレータに初期値を与え、累積和を生成する
//!
//! ## 計算量
//! $O(N)$
//!
use algebra::*;
use prelude::*;
use range_traits::*;

#[snippet(name = "cumulative-sum", doc_hidden)]
pub use cumulative_sum_impl::CumulativeSum;
#[snippet(name = "cumulative-sum", doc_hidden)]
mod cumulative_sum_impl {
    use super::{AbelianGroup, Add, FromIterator, Magma, Monoid, RangeProduct, ToBounds};
    pub struct CumulativeSum<A: Magma> {
        n: usize,
        ret: Vec<A::M>,
    }

    impl<A: Magma> CumulativeSum<A>
    where
        A::M: Clone,
    {
        /// # sum of $[0, i)$
        pub fn sum(&self, i: usize) -> A::M {
            assert!(i < self.n);
            self.ret[i].clone()
        }
    }

    impl<A: AbelianGroup> RangeProduct<usize> for CumulativeSum<A> {
        type Magma = A;
        fn product<R: ToBounds<usize>>(&self, range: R) -> A::M {
            let (a, b) = range.lr();
            if b == 0 {
                A::unit()
            } else if a == 0 {
                self.sum(b)
            } else {
                A::op(&self.sum(b), &A::inv(&self.sum(a)))
            }
        }
    }

    impl<A, T> FromIterator<T> for CumulativeSum<A>
    where
        A: Monoid<M = T>,
        T: Clone + Add<Output = T>,
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut ret = vec![A::unit()];
            for t in iter {
                ret.push(ret[ret.len() - 1].clone() + t);
            }
            let n = ret.len();
            Self { n, ret }
        }
    }
}
