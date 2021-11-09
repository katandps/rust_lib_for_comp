//! # 順列の辞書順列挙
//! 順列を与えるとそれを始点として、辞書順で次の順列を得る
//! ## 使用方法
//! - `size: usize`を与えると[0, size)の順列を辞書順に得るIteratorとなる
//! - `src: &Vec<T>` を与えると重複を考慮した順列を得る
//!
use crate::prelude::*;

#[snippet(name = "next-permutation", doc_hidden)]
#[derive(Clone, Debug)]
pub struct NextPermutation<T>(Option<Vec<T>>);
impl<T: Clone + Debug + PartialOrd> Iterator for NextPermutation<T> {
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
            }
        } else {
            self.0 = None
        };
        ret
    }
}

#[snippet(name = "next-permutation", doc_hidden)]
impl From<usize> for NextPermutation<usize> {
    fn from(size: usize) -> Self {
        NextPermutation(Some((0..size).collect()))
    }
}

#[snippet(name = "next-permutation", doc_hidden)]
impl<T: Clone + PartialOrd> From<&Vec<T>> for NextPermutation<T> {
    fn from(src: &Vec<T>) -> Self {
        NextPermutation(Some(src.to_vec()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut s = 0;
        for p in NextPermutation::from(10) {
            for pi in p {
                s += pi;
            }
        }
        assert_eq!(s, 3628800 * 45);
    }

    #[test]
    fn include_same_value() {
        let v = vec![1, 2, 2, 3];
        assert_eq!(12, NextPermutation::from(&v).count());
    }
}
