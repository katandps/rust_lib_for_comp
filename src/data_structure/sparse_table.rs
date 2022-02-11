//! # Sparse Table
//! 静的データ構造
//! 事前計算 $`O(N \log N)`$で、foldを$`O(1)`$で計算できる
//!
//! Bandが載る
//! ## verify
//! [Static RMQ](https://judge.yosupo.jp/submission/64144)
use crate::algebra::Band;
use crate::prelude::*;

#[snippet(name = "sparse-table", doc_hidden)]
#[derive(Clone)]
pub struct SparseTable<B: Band> {
    size: usize,
    table: Vec<Vec<B::M>>,
}

#[snippet(name = "sparse-table", doc_hidden)]
impl<B: Band> From<&[B::M]> for SparseTable<B> {
    fn from(v: &[B::M]) -> Self {
        let size = v.len();
        let l = v.len();
        let lg = 63 - l.leading_zeros();
        let mut table = vec![Vec::new(); lg as usize + 1];
        table[0] = v.to_vec();
        let mut k = 1;
        while 1 << k <= size {
            table[k] = (0..=size - (1 << k))
                .map(|i| B::op(&table[k - 1][i], &table[k - 1][i + (1 << (k - 1))]))
                .collect();
            k += 1;
        }
        Self { size, table }
    }
}

#[snippet(name = "sparse-table", doc_hidden)]
impl<B: Band> SparseTable<B> {
    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> B::M {
        let (l, r) = to_lr(&range, self.size);
        let lg = 63 - (r - l).leading_zeros();
        B::op(
            &self.table[lg as usize][l],
            &self.table[lg as usize][r - (1 << lg)],
        )
    }
}
#[snippet(name = "sparse-table", doc_hidden)]
impl<B: Band> Debug for SparseTable<B>
where
    B::M: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for i in 0..self.size {
            writeln!(f, "{:?}", self.query(i..=i))?;
        }
        Ok(())
    }
}
