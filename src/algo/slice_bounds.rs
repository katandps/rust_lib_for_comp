//! # スライス上の二分探索
//! 用いる関数を適用した結果について昇順になるよう、スライスをソートしておく必要がある
//! ## 計算量
//! $`\logN` (Nはスライスの長さ)$
use crate::prelude::*;

#[snippet(name = "slice-bounds", doc_hidden)]
/// # 要素
pub trait SliceBounds {
    type Item: Ord;
    /// k以上の要素となる最小のindexを返す
    fn lower_bound(&self, k: &Self::Item) -> usize;
    /// kより大きい要素となる最小のindexを返す
    fn upper_bound(&self, k: &Self::Item) -> usize;
}
#[snippet(name = "slice-bounds", doc_hidden)]
/// # 比較関数
pub trait SliceBoundsBy {
    type Item;
    fn lower_bound_by<F: FnMut(&Self::Item) -> Ordering>(&self, f: F) -> usize;
    fn upper_bound_by<F: FnMut(&Self::Item) -> Ordering>(&self, f: F) -> usize;
}
#[snippet(name = "slice-bounds", doc_hidden)]
/// # 関数を適用した結果
pub trait SliceBoundsByKey {
    type Item;
    fn lower_bound_by_key<K: Ord, F: FnMut(&Self::Item) -> K>(&self, k: &K, f: F) -> usize;
    fn upper_bound_by_key<K: Ord, F: FnMut(&Self::Item) -> K>(&self, k: &K, f: F) -> usize;
}
#[snippet(name = "slice-bounds", doc_hidden)]
impl<T: Ord> SliceBounds for [T] {
    type Item = T;
    fn lower_bound(&self, k: &Self::Item) -> usize {
        self.lower_bound_by(|p| p.cmp(k))
    }
    fn upper_bound(&self, k: &T) -> usize {
        self.upper_bound_by(|p| p.cmp(k))
    }
}
#[snippet(name = "slice-bounds", doc_hidden)]
impl<T> SliceBoundsBy for [T] {
    type Item = T;
    fn lower_bound_by<F: FnMut(&T) -> Ordering>(&self, mut f: F) -> usize {
        self.binary_search_by(|p| f(p).then(Ordering::Greater))
            .unwrap_err()
    }
    fn upper_bound_by<F: FnMut(&T) -> Ordering>(&self, mut f: F) -> usize {
        self.binary_search_by(|p| f(p).then(Ordering::Less))
            .unwrap_err()
    }
}
#[snippet(name = "slice-bounds", doc_hidden)]
impl<T> SliceBoundsByKey for [T] {
    type Item = T;
    fn lower_bound_by_key<K: Ord, F: FnMut(&T) -> K>(&self, k: &K, mut f: F) -> usize {
        self.lower_bound_by(|p| f(p).cmp(k))
    }
    fn upper_bound_by_key<K: Ord, F: FnMut(&T) -> K>(&self, k: &K, mut f: F) -> usize {
        self.upper_bound_by(|p| f(p).cmp(k))
    }
}

#[test]
fn test() {
    let src = vec![1, 1, 1, 2, 2, 5, 8];
    for i in 0..=10 {
        println!("{} {} {}", i, src.lower_bound(&i), src.upper_bound(&i));
    }
}
