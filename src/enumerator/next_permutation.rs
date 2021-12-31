//! # 順列の辞書順列挙
//! 辞書順で次の順列を得るIteratorの実装
//! ## 使用方法
//! - `From<usize>`: [0, size)の順列を辞書順に得る
//! - `From<&[T]>`: スライスの順列を辞書順に得る
//!     - 辞書順で入力以降のもののみ得られる
use crate::prelude::*;

#[snippet(name = "next-permutation", doc_hidden)]
#[derive(Clone, Debug)]
pub struct NextPermutation<T>(Option<Vec<T>>);
#[snippet(name = "next-permutation", doc_hidden)]
impl<T: Clone + PartialOrd> Iterator for NextPermutation<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Vec<T>> {
        let ret = self.0.clone();
        if let Some(v) = &mut self.0 {
            if v.len() >= 2 {
                let mut i = v.len() - 1;
                while i > 0 {
                    if v[i - 1] < v[i] {
                        let j = (0..v.len()).rev().find(|j| v[*j] > v[i - 1]).unwrap();
                        v.swap(i - 1, j);
                        v[i..].reverse();
                        break;
                    }
                    i -= 1;
                }
                if i == 0 {
                    self.0 = None
                }
            } else {
                self.0 = None
            }
        } else {
            self.0 = None
        };
        ret
    }
}
/// [0, size)の順列を辞書順に得るIteratorとなる
/// ```
/// # use rust_lib_for_comp::enumerator::next_permutation::*;
/// let mut s = 0;
/// for p in NextPermutation::from(10) {
///     for pi in p {
///         s += pi;
///     }
/// }
/// assert_eq!(s, 3628800 * 45);
/// ```
#[snippet(name = "next-permutation", doc_hidden)]
impl From<usize> for NextPermutation<usize> {
    fn from(size: usize) -> Self {
        NextPermutation(Some((0..size).collect()))
    }
}
/// 重複を考慮した順列を辞書順に得るIteratorとなる
/// ```
/// # use rust_lib_for_comp::enumerator::next_permutation::*;
/// let v = vec![1, 2, 2, 3];
/// assert_eq!(12, NextPermutation::from(&v[..]).count());
/// ```
#[snippet(name = "next-permutation", doc_hidden)]
impl<T: Clone + PartialOrd> From<&[T]> for NextPermutation<T> {
    fn from(src: &[T]) -> Self {
        NextPermutation(Some(src.to_vec()))
    }
}
