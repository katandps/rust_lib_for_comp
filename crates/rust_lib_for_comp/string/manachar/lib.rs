//! # 最長回文長 Manachar's Algorithm
//!
//! ## 計算量
//! $O(N)$

use crate::prelude::*;
#[codesnip::entry("manachar")]
pub use manachar_impl::Manachar;
#[codesnip::entry("manachar")]
mod manachar_impl {
    use super::Debug;
    pub struct Manachar;
    impl Manachar {
        /// # 文字 $i$ を中心とした(奇数長の)最大回文長
        pub fn manachar_item_center<T: Eq>(src: &[T]) -> Vec<usize> {
            Self::logic(src).into_iter().map(|r| r * 2 - 1).collect()
        }
        /// # 最大回文長
        pub fn manachar<T: Eq + Clone + Debug>(src: &[T]) -> Vec<usize> {
            if src.is_empty() {
                return Vec::new();
            }
            let mut v = vec![src[0].clone(); src.len() * 2 - 1];
            for i in 0..src.len() {
                v[i * 2] = src[i].clone();
                if i * 2 + 1 < v.len() {
                    v[i * 2 + 1] = src[0].clone();
                }
            }
            Self::logic(&v)
                .into_iter()
                .enumerate()
                .map(|(i, r)| if (i ^ r) & 1 == 0 { r - 1 } else { r })
                .collect()
        }

        fn logic<T: Eq>(src: &[T]) -> Vec<usize> {
            let (mut i, mut j, mut rad) = (0, 0, vec![0; src.len()]);
            while i < src.len() {
                while i >= j && i + j < src.len() && src[i - j] == src[i + j] {
                    j += 1;
                }
                rad[i] = j;
                let mut k = 1;
                while i >= k && k + rad[i - k] < j {
                    rad[i + k] = rad[i - k];
                    k += 1;
                }
                i += k;
                j -= k;
            }
            rad
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let empty = vec![1];
        assert_eq!(Manachar::manachar(&empty[0..0]), Vec::new());
    }

    #[test]
    fn test_item_center() {
        let src: Vec<_> = "abaaababa".chars().collect();
        let res: Vec<usize> = Manachar::manachar_item_center(&src);
        assert_eq!(vec![1, 3, 1, 7, 1, 3, 5, 3, 1], res);
    }

    #[test]
    fn test() {
        let src: Vec<_> = "abcbcba".chars().collect();
        let res: Vec<usize> = Manachar::manachar(&src);
        assert_eq!(vec![1, 0, 1, 0, 3, 0, 7, 0, 3, 0, 1, 0, 1], res);
    }
}
