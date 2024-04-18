//! # 累積和
//!
//! イテレータに初期値を与え、累積和を生成する
//!
//! ## 計算量
//! $O(N)$
//!
use crate::algebra::*;
use crate::range_traits::*;

#[codesnip::entry("cumulative-sum")]
pub use cumulative_sum_impl::CumulativeSum;
#[codesnip::entry("cumulative-sum", include("algebra", "prelude", "range-traits"))]
mod cumulative_sum_impl {
    use super::{AbelianGroup, Magma, RangeProductMut, ToBounds};
    pub struct CumulativeSum<A: Magma> {
        n: usize,
        ret: Vec<A::M>,
        abelian_group: A,
    }

    impl<A: Magma> CumulativeSum<A> {
        pub fn build<I: IntoIterator<Item = A::M>>(
            iter: I,
            mut abelian_group: A,
            initial: A::M,
        ) -> Self {
            let mut ret = vec![initial];
            for t in iter {
                ret.push(abelian_group.op(&ret[ret.len() - 1], &t));
            }
            let n = ret.len();
            Self {
                n,
                ret,
                abelian_group,
            }
        }

        /// # sum of $[0, i)$
        pub fn sum(&self, i: usize) -> A::M {
            assert!(i < self.n);
            self.ret[i].clone()
        }
    }

    impl<A: AbelianGroup> RangeProductMut<usize> for CumulativeSum<A> {
        type Magma = A;
        fn product<R: ToBounds<usize>>(&mut self, range: R) -> A::M {
            let (a, b) = range.lr();
            if b == 0 {
                A::unit()
            } else if a == 0 {
                self.sum(b)
            } else {
                self.abelian_group.op(&self.sum(b), &A::inv(&self.sum(a)))
            }
        }
    }
}
