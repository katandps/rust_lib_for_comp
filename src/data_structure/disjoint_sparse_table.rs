//! # DisjointSparseTable
//! 静的データ構造 半群への区間クエリを構築$O(N \log N)$、クエリ$O(1)$で処理する
//!
//! ## verify
//! [Static RMQ](https://judge.yosupo.jp/submission/65663)
use crate::algebra::SemiGroup;
use crate::prelude::*;

#[snippet(name = "disjoint-sparse-table", doc_hidden)]
pub struct DisjointSparseTable<S: SemiGroup> {
    _len: usize,
    table: Vec<Vec<S::M>>,
    lookup: Vec<usize>,
}

#[snippet(name = "disjoint-sparse-table", doc_hidden)]
impl<S: SemiGroup> From<&[S::M]> for DisjointSparseTable<S> {
    fn from(src: &[S::M]) -> Self {
        let len = src.len();
        let mut log = 0;
        while 1 << log <= len {
            log += 1;
        }
        let mut table = vec![src.to_vec()];
        (1..log).for_each(|i| {
            let mut v = vec![None; len + 1];
            let shift = 1 << i;
            let mut j = 0;
            while j < len {
                let t = min(j + shift, len + 1);
                v[t - 1] = Some(src[t - 1].clone());
                (j..t - 1)
                    .rev()
                    .for_each(|k| v[k] = v[k + 1].clone().map(|vk| S::op(&src[k], &vk)));
                if len <= t {
                    break;
                }
                v[t] = Some(src[t].clone());
                let r = min(t + shift, len + 1);
                (t + 1..r).for_each(|k| v[k] = v[k - 1].clone().map(|vk| S::op(&vk, &src[k])));
                j += shift << 1;
            }
            table.push(v.into_iter().flatten().collect());
        });
        let mut lookup = vec![0; 1 << log];
        (2..lookup.len()).for_each(|i| lookup[i] = lookup[i >> 1] + 1);
        Self {
            _len: len,
            table,
            lookup,
        }
    }
}

#[snippet(name = "disjoint-sparse-table", doc_hidden)]
impl<S: SemiGroup> DisjointSparseTable<S> {
    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> S::M {
        let (l, r) = range.to_lr();
        let p = self.lookup[l ^ r];
        S::op(&self.table[p][l], &self.table[p][r])
    }
}
