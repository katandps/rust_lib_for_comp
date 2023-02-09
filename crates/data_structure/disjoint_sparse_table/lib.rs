//! # DisjointSparseTable
//! 静的データ構造 半群への区間クエリを構築$O(N \log N)$、クエリ$O(1)$で処理する
//!
//! ## verify
//! [Static RMQ](https://judge.yosupo.jp/submission/124820)
use algebra::*;
use prelude::*;
use range_traits::*;
use string_util::JoinTrait;

#[snippet(name = "disjoint-sparse-table", doc_hidden)]
pub use disjoint_sparse_table_impl::DisjointSparseTable;

#[snippet(name = "disjoint-sparse-table", doc_hidden)]
mod disjoint_sparse_table_impl {
    use super::{
        min, Debug, Display, Formatter, JoinTrait, RangeBounds, RangeProduct, SemiGroup, ToLR,
    };

    pub struct DisjointSparseTable<S: SemiGroup> {
        pub len: usize,
        table: Vec<Vec<S::M>>,
        lookup: Vec<usize>,
    }

    impl<S: SemiGroup> From<&[S::M]> for DisjointSparseTable<S> {
        fn from(src: &[S::M]) -> Self {
            let len = (src.len() + 1).next_power_of_two();
            let log = len.trailing_zeros() as usize;
            let mut table = vec![src.to_vec()];
            (1..log).for_each(|i| {
                let mut v = vec![None; len + 1];
                let shift = 1 << i;
                let mut j = 0;
                while j < len {
                    let t = min(j + shift, len + 1);
                    v[t - 1] = src.get(t - 1).cloned();
                    (j..t - 1).rev().for_each(|k| {
                        v[k] = v[k + 1]
                            .as_ref()
                            .and_then(|vk| src.get(k).map(|sk| S::op(sk, vk)))
                    });
                    if len <= t {
                        break;
                    }
                    v[t] = src.get(t).cloned();
                    let r = min(t + shift, len + 1);
                    (t + 1..r).for_each(|k| {
                        v[k] = v[k - 1]
                            .as_ref()
                            .and_then(|vk| src.get(k).map(|sk| S::op(vk, sk)))
                    });
                    j += shift << 1;
                }
                table.push(v.into_iter().flatten().collect());
            });
            let mut lookup = vec![0; 1 << log];
            (2..lookup.len()).for_each(|i| lookup[i] = lookup[i >> 1] + 1);
            Self {
                len: src.len(),
                table,
                lookup,
            }
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(1)$
    impl<S: SemiGroup> RangeProduct<usize> for DisjointSparseTable<S> {
        type Magma = S;
        fn product<R: RangeBounds<usize>>(&self, range: R) -> S::M {
            let (l, mut r) = range.to_lr();
            assert!(l < r);
            // l..=rに変換
            r -= 1;
            if l == r {
                self.table[0][l].clone()
            } else {
                let p = self.lookup[l ^ r];
                S::op(&self.table[p][l], &self.table[p][r])
            }
        }
    }

    impl<B: SemiGroup> Debug for DisjointSparseTable<B>
    where
        B::M: Display,
    {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "\n{}",
                (0..self.len).map(|i| self.product(i..=i)).join(" ")
            )
        }
    }
}

#[test]
fn test() {
    use addition::Addition;

    let src = vec![1i64, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    let dst = DisjointSparseTable::<Addition<i64>>::from(&src[..]);

    for i in 0..src.len() {
        for j in i + 1..=src.len() {
            let mut m = 0;
            for k in i..j {
                m = Addition::op(&m, &src[k]);
            }
            assert_eq!(m, dst.product(i..j), "{}..{}", i, j);
        }
    }
}
