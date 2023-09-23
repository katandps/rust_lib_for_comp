//! # Sparse Table
//! 静的データ構造
//! 事前計算 $O(N\log N)$で、区間積を$O(1)$で計算できる
//!
//! Bandが載る
//! ## verify
//! [Static RMQ](https://judge.yosupo.jp/submission/64144)
use algebra::Band;
use prelude::*;
use range_traits::{RangeProduct, ToBounds};
use string_util::JoinTrait;

#[snippet(name = "sparse-table", doc_hidden)]
pub use sparse_table_impl::SparseTable;
#[snippet(name = "sparse-table", doc_hidden)]
mod sparse_table_impl {
    use super::{Band, Debug, Display, Formatter, JoinTrait, RangeProduct, ToBounds};

    #[derive(Clone)]
    pub struct SparseTable<B: Band> {
        pub len: usize,
        table: Vec<Vec<B::M>>,
    }

    impl<B: Band> From<&[B::M]> for SparseTable<B> {
        fn from(v: &[B::M]) -> Self {
            let len = v.len();
            let l = v.len();
            let lg = 63 - l.leading_zeros();
            let mut table = vec![Vec::new(); lg as usize + 1];
            table[0] = v.to_vec();
            let mut k = 1;
            while 1 << k <= len {
                table[k] = (0..=len - (1 << k))
                    .map(|i| B::op(&table[k - 1][i], &table[k - 1][i + (1 << (k - 1))]))
                    .collect();
                k += 1;
            }
            Self { len, table }
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(1)$
    impl<B: Band> RangeProduct<usize> for SparseTable<B> {
        type Magma = B;
        fn product<R: ToBounds<usize>>(&self, range: R) -> B::M {
            let (l, r) = range.lr();
            let lg = 63 - (r - l).leading_zeros();
            B::op(
                &self.table[lg as usize][l],
                &self.table[lg as usize][r - (1 << lg)],
            )
        }
    }

    impl<B: Band> Debug for SparseTable<B>
    where
        B::M: Display,
    {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            writeln!(
                f,
                "\n{}",
                (0..self.len).map(|i| self.product(i..=i)).join(" ")
            )
        }
    }
}

#[test]
fn test() {
    use algebra::Magma;
    use minimization::Minimization;

    let src = vec![1i64, 5, 6, 2, 3, 9, 7, 4, 0, 8];
    let sparse_table = SparseTable::<Minimization<i64>>::from(&src[..]);
    for i in 0..src.len() {
        for j in i + 1..=src.len() {
            let mut m = 10;
            for k in i..j {
                m = Minimization::op(&m, &src[k]);
            }
            assert_eq!(m, sparse_table.product(i..j));
        }
    }
}

#[test]
fn debug_test() {
    use minimization::Minimization;
    let src = vec![1i64, 5, 6, 2, 3, 9, 7, 4, 0, 8];
    let sparse_table = SparseTable::<Minimization<i64>>::from(&src[..]);
    let debug = format!("{:?}", sparse_table);
    assert_eq!(debug.as_str(), "\n1 5 6 2 3 9 7 4 0 8\n");
}
