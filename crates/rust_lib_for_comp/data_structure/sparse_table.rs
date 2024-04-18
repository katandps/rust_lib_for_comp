//! # Sparse Table
//! 静的データ構造
//! 事前計算 $O(N\log N)$で、区間積を$O(1)$で計算できる
//!
//! Bandが載る
use crate::algebra::Band;
use crate::prelude::*;
use crate::range_traits::{RangeProductMut, ToBounds};
use crate::util::string_util::JoinTrait;

#[codesnip::entry("sparse-table")]
pub use sparse_table_impl::SparseTable;
#[codesnip::entry(
    "sparse-table",
    include("algebra", "prelude", "range-traits", "string-util")
)]
mod sparse_table_impl {
    use super::{Band, Display, JoinTrait, RangeProductMut, ToBounds};

    #[derive(Clone)]
    pub struct SparseTable<B: Band> {
        pub len: usize,
        table: Vec<Vec<B::M>>,
        band: B,
    }

    impl<B: Band> SparseTable<B> {
        pub fn build(v: &[B::M], mut band: B) -> Self {
            let len = v.len();
            let l = v.len();
            let lg = 63 - l.leading_zeros();
            let mut table = vec![Vec::new(); lg as usize + 1];
            table[0] = v.to_vec();
            let mut k = 1;
            while 1 << k <= len {
                table[k] = (0..=len - (1 << k))
                    .map(|i| band.op(&table[k - 1][i], &table[k - 1][i + (1 << (k - 1))]))
                    .collect();
                k += 1;
            }
            Self { len, table, band }
        }

        pub fn into_string(mut self) -> String
        where
            B::M: Display,
        {
            (0..self.len).map(|i| self.product(i..=i)).join(" ")
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(1)$
    impl<B: Band> RangeProductMut<usize> for SparseTable<B> {
        type Magma = B;
        fn product<R: ToBounds<usize>>(&mut self, range: R) -> B::M {
            let (l, r) = range.lr();
            let lg = 63 - (r - l).leading_zeros();
            self.band.op(
                &self.table[lg as usize][l],
                &self.table[lg as usize][r - (1 << lg)],
            )
        }
    }
}

#[test]
fn test() {
    use crate::algebra::binary_operation::minimization::Minimization;
    use crate::algebra::Magma;

    let src = [1i64, 5, 6, 2, 3, 9, 7, 4, 0, 8];
    let mut band = Minimization::default();
    let mut sparse_table = SparseTable::build(&src[..], band.clone());
    for i in 0..src.len() {
        for j in i + 1..=src.len() {
            let m = (i..j).fold(10, |x, i| band.op(&x, &src[i]));
            assert_eq!(m, sparse_table.product(i..j));
        }
    }
}

#[test]
fn debug_test() {
    use crate::algebra::binary_operation::minimization::Minimization;
    let src = [1i64, 5, 6, 2, 3, 9, 7, 4, 0, 8];
    let sparse_table = SparseTable::build(&src[..], Minimization::default());
    assert_eq!(sparse_table.into_string().as_str(), "1 5 6 2 3 9 7 4 0 8");
}
