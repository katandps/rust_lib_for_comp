//! # 順列の辞書順列挙
//! 辞書順で次の順列を得るIteratorの実装
//! ## 使用方法
//! - `From<usize>`: [0, size)の順列を辞書順に得る
//! - `From<&[T]>`: スライスの順列を辞書順に得る
//!     - 辞書順で入力以降のもののみ得られる
use prelude::*;

pub use self::next_permutation_impl::{next_permutation_from_i, next_permutation_from_slice};
#[codesnip::entry("next-permutation", doc_hidden)]
mod next_permutation_impl {
    /// [0, size)の順列を辞書順に得るIteratorとなる
    /// ```
    /// # use next_permutation::*;
    /// let mut s = 0;
    /// for p in next_permutation_from_i(5) {
    ///     for pi in p {
    ///         s += pi;
    ///     }
    /// }
    /// assert_eq!(s, 120 * 10);
    /// ```
    pub fn next_permutation_from_i(size: usize) -> impl Iterator<Item = Vec<usize>> {
        std::iter::successors(Some((0..size).collect::<Vec<_>>()), move |v| {
            let mut v = v.to_vec();
            if v.len() >= 2 {
                for i in (1..v.len()).rev() {
                    if v[i - 1] < v[i] {
                        let j = (0..v.len()).rev().find(|j| v[*j] > v[i - 1]).unwrap();
                        v.swap(i - 1, j);
                        v[i..].reverse();
                        return Some(v.to_vec());
                    }
                }
            }
            None
        })
    }
    /// 重複を考慮した順列を辞書順に得るIteratorとなる
    /// ```
    /// # use next_permutation::*;
    /// let v = vec![1, 2, 2, 3];
    /// assert_eq!(12, next_permutation_from_slice(&v[..]).count());
    /// ```
    pub fn next_permutation_from_slice<T: Clone + PartialOrd>(
        src: &[T],
    ) -> impl Iterator<Item = Vec<T>> {
        std::iter::successors(Some(src.to_vec()), move |v| {
            let mut v = v.to_vec();
            if v.len() >= 2 {
                for i in (1..v.len()).rev() {
                    if v[i - 1] < v[i] {
                        let j = (0..v.len()).rev().find(|j| v[*j] > v[i - 1]).unwrap();
                        v.swap(i - 1, j);
                        v[i..].reverse();
                        return Some(v.to_vec());
                    }
                }
            }
            None
        })
    }
}

#[test]
fn test() {
    let v = vec![1, 2, 3];
    let mut b = Vec::new();
    for p in next_permutation_from_slice(&v[..]) {
        b.push(p);
    }
    assert_eq!(
        b,
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
    )
}
