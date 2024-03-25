//! # スライス上の二分探索
//! 用いる関数を適用した結果について昇順になるよう、スライスをソートしておく必要がある
//! ## 計算量
//! $ \log N (Nはスライスの長さ)$
use crate::prelude::*;

#[codesnip::entry("slice-bounds")]
pub use slice_traits::{SliceBounds, SliceBoundsBy, SliceBoundsByKey};
#[codesnip::entry("slice-bounds", include("prelude"))]
mod slice_traits {
    use super::Ordering;
    /// # 要素
    pub trait SliceBounds {
        type Item: Ord;
        /// k以上の要素となる最小のindexを返す 存在しない場合はスライスの長さを返す
        fn lower_bound(&self, k: &Self::Item) -> usize;
        /// kより大きい要素となる最小のindexを返す
        fn upper_bound(&self, k: &Self::Item) -> usize;
        fn is_empty(&self) -> bool {
            self.len() == 0
        }
        fn len(&self) -> usize;
        /// k未満の要素の数
        fn less_count(&self, k: &Self::Item) -> usize {
            self.lower_bound(k)
        }
        /// k以下の要素の数
        fn less_eq_count(&self, k: &Self::Item) -> usize {
            self.upper_bound(k)
        }
        /// kより大きい要素の数
        fn more_count(&self, k: &Self::Item) -> usize {
            self.len() - self.less_eq_count(k)
        }
        /// k以上の要素の数
        fn more_eq_count(&self, k: &Self::Item) -> usize {
            self.len() - self.less_count(k)
        }
    }
    /// # 比較関数
    pub trait SliceBoundsBy {
        type Item;
        fn lower_bound_by<F: FnMut(&Self::Item) -> Ordering>(&self, f: F) -> usize;
        fn upper_bound_by<F: FnMut(&Self::Item) -> Ordering>(&self, f: F) -> usize;
    }
    /// # 関数を適用した結果
    pub trait SliceBoundsByKey {
        type Item;
        fn lower_bound_by_key<K: Ord, F: FnMut(&Self::Item) -> K>(&self, k: &K, f: F) -> usize;
        fn upper_bound_by_key<K: Ord, F: FnMut(&Self::Item) -> K>(&self, k: &K, f: F) -> usize;
    }
    impl<T: Ord> SliceBounds for [T] {
        type Item = T;
        fn lower_bound(&self, k: &T) -> usize {
            self.lower_bound_by(|p| p.cmp(k))
        }
        fn upper_bound(&self, k: &T) -> usize {
            self.upper_bound_by(|p| p.cmp(k))
        }
        fn len(&self) -> usize {
            self.len()
        }
    }
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
    impl<T> SliceBoundsByKey for [T] {
        type Item = T;
        fn lower_bound_by_key<K: Ord, F: FnMut(&T) -> K>(&self, k: &K, mut f: F) -> usize {
            self.lower_bound_by(|p| f(p).cmp(k))
        }
        fn upper_bound_by_key<K: Ord, F: FnMut(&T) -> K>(&self, k: &K, mut f: F) -> usize {
            self.upper_bound_by(|p| f(p).cmp(k))
        }
    }
}
#[test]
fn test() {
    let src = [1, 1, 1, 2, 2, 5, 8];
    let expect_lower = [0, 0, 3, 5, 5, 5, 6, 6, 6, 7, 7];
    let expect_upper = [0, 3, 5, 5, 5, 6, 6, 6, 7, 7, 7];

    for i in 0..=10 {
        assert_eq!(expect_lower[i], src.lower_bound(&i));
        assert_eq!(expect_upper[i], src.upper_bound(&i));
    }
}
