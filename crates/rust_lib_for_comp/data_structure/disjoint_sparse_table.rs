//! # DisjointSparseTable
//! 静的データ構造 半群への区間クエリを構築$O(N \log N)$、クエリ$O(1)$で処理する
//!
//! ## verify
//! [Static RMQ](https://judge.yosupo.jp/submission/124820)
use crate::algebra::*;
use crate::prelude::*;
use crate::range_traits::*;
use crate::util::string_util::JoinTrait;

#[codesnip::entry("disjoint-sparse-table")]
pub use disjoint_sparse_table_impl::DisjointSparseTable;

#[codesnip::entry(
    "disjoint-sparse-table",
    include("algebra", "prelude", "range-traits", "string-util")
)]
mod disjoint_sparse_table_impl {
    use super::{min, Display, JoinTrait, RangeProductMut, SemiGroup, ToBounds};

    pub struct DisjointSparseTable<S: SemiGroup> {
        pub len: usize,
        table: Vec<Vec<S::M>>,
        lookup: Vec<usize>,
        semigroup: S,
    }

    impl<S: SemiGroup> DisjointSparseTable<S> {
        pub fn build(src: &[S::M], mut semigroup: S) -> Self {
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
                            .and_then(|vk| src.get(k).map(|sk| semigroup.op(sk, vk)))
                    });
                    if len <= t {
                        break;
                    }
                    v[t] = src.get(t).cloned();
                    let r = min(t + shift, len + 1);
                    (t + 1..r).for_each(|k| {
                        v[k] = v[k - 1]
                            .as_ref()
                            .and_then(|vk| src.get(k).map(|sk| semigroup.op(vk, sk)))
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
                semigroup,
            }
        }

        pub fn into_string(mut self) -> String
        where
            S::M: Display,
        {
            (0..self.len).map(|i| self.product(i..=i)).join(" ")
        }
    }

    /// # 区間の総積
    /// ## 計算量
    /// $O(1)$
    impl<S: SemiGroup> RangeProductMut<usize> for DisjointSparseTable<S> {
        type Magma = S;
        fn product<R: ToBounds<usize>>(&mut self, range: R) -> S::M {
            let (l, mut r) = range.lr();
            assert!(l < r);
            // l..=rに変換
            r -= 1;
            if l == r {
                self.table[0][l].clone()
            } else {
                let p = self.lookup[l ^ r];
                self.semigroup.op(&self.table[p][l], &self.table[p][r])
            }
        }
    }
}

#[test]
fn test() {
    use crate::algebra::binary_operation::addition::Addition;

    let src = [1i64, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    let mut dst = DisjointSparseTable::build(&src[..], Addition::default());

    for i in 0..src.len() {
        for j in i + 1..=src.len() {
            let m = (i..j).fold(0, |x, i| x + src[i]);
            assert_eq!(m, dst.product(i..j), "{}..{}", i, j);
        }
    }
}
