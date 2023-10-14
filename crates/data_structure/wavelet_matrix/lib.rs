//! # ウェーブレット行列
//! ## 概要
//! 正整数列について、構築$O(N\log V)$で$O(\log V)$ or $O(\log^2 V)$のクエリを提供する
//!
//! ## 使い方
//! ```
//! # use wavelet_matrix::*;
//! let src = vec![5, 7, 3, 4, 2, 9];
//! let wm = WaveletMatrix::from(src.clone());
//! src.iter()
//!     .enumerate()
//!     .for_each(|(i, &src)| assert_eq!(src, wm.access(i)));
//! ```
//!
//! ## dependencies
//! [完備辞書](succinct_indexable_dictionaries)
//!
//! ## 計算量
//! - 構築: $O(N \log V)$
//! - クエリ: $O( \log^2 V)$
//!
use fxhasher::HashMap;
use prelude::*;
use range_traits::ToBounds;
use succinct_indexable_dictionaries::{SIDBuilder, SID};

#[snippet(name = "wavelet-matrix", doc_hidden)]
#[snippet(include = "succinct-indexable-dictionaries")]
pub use wavelet_matrix_impl::WaveletMatrix;

#[snippet(name = "wavelet-matrix", doc_hidden)]
mod wavelet_matrix_impl {
    use super::{HashMap, SIDBuilder, ToBounds, SID};

    #[derive(Debug)]
    pub struct WaveletMatrix {
        size: usize,
        depth: usize,
        // bitごとにsortした索引 小さい桁から入っている
        matrix: Vec<SID>,
        // 各索引の境界
        mid: Vec<usize>,
        /// ソートが終わった後の各値の最左位置
        id: HashMap<u64, usize>,
    }

    impl<T: Clone + Into<u64>, I: IntoIterator<Item = T>> From<I> for WaveletMatrix {
        fn from(src: I) -> Self {
            let mut src = src.into_iter().map(|si| si.into()).collect::<Vec<u64>>();
            let size = src.len();
            let depth = src
                .iter()
                .map(|si| Self::DEPTH - si.leading_zeros())
                .max()
                .unwrap_or(0) as usize;
            let mut matrix = Vec::with_capacity(depth);
            let mut mid = Vec::with_capacity(depth);
            let (mut l, mut r) = (Vec::with_capacity(size), Vec::with_capacity(size));
            (0..depth).rev().for_each(|level| {
                l.clear();
                r.clear();
                let mut builder = SIDBuilder::new(size);
                (0..size).for_each(|i| {
                    if src[i] >> level & 1 > 0 {
                        builder.set(i);
                        r.push(src[i]);
                    } else {
                        l.push(src[i]);
                    }
                });
                mid.push(l.len());
                matrix.push(builder.build());
                src.clear();
                src.append(&mut l);
                src.append(&mut r);
            });
            matrix.reverse();
            mid.reverse();
            let id = src
                .iter()
                .enumerate()
                .rev() // 先頭のindexを保存する
                .map(|(i, si)| (*si, i))
                .collect();
            Self {
                size,
                depth,
                matrix,
                mid,
                id,
            }
        }
    }

    impl WaveletMatrix {
        /// $2^{DEPTH}$まで格納できる
        const DEPTH: u32 = u64::BITS;
        /// # Indexを指定して要素を取得
        /// ## 計算量
        /// $O(\log V)$
        pub fn access(&self, mut index: usize) -> u64 {
            let mut ret = 0;
            (0..self.depth).rev().for_each(|level| {
                let f = self.matrix[level].access(index);
                if f {
                    ret |= 1u64 << level
                }
                index = self.matrix[level].rank(index, f) + self.mid[level] * usize::from(f);
            });
            ret
        }

        // [l, r)の範囲について、level-bit目がbであるようなものの範囲を返す
        fn succ(&self, b: bool, l: usize, r: usize, level: usize) -> (usize, usize) {
            (
                self.matrix[level].rank(l, b) + self.mid[level] * usize::from(b),
                self.matrix[level].rank(r, b) + self.mid[level] * usize::from(b),
            )
        }

        /// # $[0 <= i < r) かつ $v\[i\] == x$ であるようなiの個数
        /// ## 計算量
        /// $O(\log V)$
        pub fn rank(&self, x: u64, r: usize) -> usize {
            let (_l, r) = (0..self.depth).rev().fold((0, r), |(l, r), level| {
                self.succ((x >> level) & 1 > 0, l, r, level)
            });
            r - self.id.get(&x).unwrap_or(&r)
        }

        /// # section内で $v\[i\] == x$ であるようなiの個数
        /// ## 計算量
        /// $O(\log V)$
        pub fn rank_section(&self, section: impl ToBounds<usize>, x: u64) -> usize {
            let (l, r) = section.lr();
            self.rank(x, r) - self.rank(x, l)
        }

        /// # 全体から1-indexedでi番目に登場するxの位置
        /// ## 計算量
        /// $O(\log^2 V)$
        pub fn select(&self, x: u64, i: usize) -> Option<usize> {
            self.id.get(&x).and_then(|&c| {
                let p: Option<usize> = (0..self.depth).try_fold(c + i, |p, level| {
                    let b = x >> level & 1 == 1;
                    self.matrix[level].select(p - self.mid[level] * usize::from(b), b)
                });
                p.map(|p| p - 1)
            })
        }

        /// # range のうち、小さい方から0-indexedでk番目の数
        /// ## 計算量
        /// $O(\log V)$
        pub fn kth_smallest(&self, section: impl ToBounds<usize>, mut k: usize) -> u64 {
            let (l, r) = section.lr();
            assert!(k < r - l);
            let mut ret = 0;
            (0..self.depth).rev().fold((l, r), |(l, r), level| {
                // 範囲内で、現在のbitが0であるものの個数
                let cnt = self.matrix[level].rank(r, false) - self.matrix[level].rank(l, false);
                if cnt <= k {
                    ret |= 1 << level;
                    k -= cnt;
                    self.succ(true, l, r, level)
                } else {
                    self.succ(false, l, r, level)
                }
            });
            ret
        }
        /// # range中で大きい方から0-indexedでk番目の数
        /// ## 計算量
        /// $O(\log V)$
        pub fn kth_largest(&self, range: impl ToBounds<usize>, k: usize) -> u64 {
            let (l, r) = range.lr();
            self.kth_smallest(range, r - l - k - 1)
        }

        /// # index_range のうち、値がupper未満のものの個数
        /// ## 計算量
        /// $O(\log V)$
        pub fn range_lower_than(&self, index_range: impl ToBounds<usize>, upper: u64) -> usize {
            if upper >= 1 << self.depth {
                return self.size;
            }
            let (l, r) = index_range.lr();
            let mut ret = 0;
            (0..self.depth).rev().fold((l, r), |(l, r), level| {
                let b = upper >> level & 1 == 1;
                if b {
                    ret += self.matrix[level].rank(r, false) - self.matrix[level].rank(l, false);
                }
                self.succ(b, l, r, level)
            });
            ret
        }

        /// # index_range, x_range内に含まれる数の個数
        ///
        /// ## 計算量
        /// $O(\log V)$
        pub fn range_frequency(
            &self,
            index_range: impl ToBounds<usize> + Clone,
            x_range: impl ToBounds<u64>,
        ) -> usize {
            let (mi, ma) = x_range.lr();
            self.range_lower_than(index_range.clone(), ma) - self.range_lower_than(index_range, mi)
        }

        /// # rangeのうち、 upperより小さい要素で最大のもの
        pub fn prev(&self, range: impl ToBounds<usize> + Clone, upper: u64) -> Option<u64> {
            let cnt = self.range_lower_than(range.clone(), upper);
            if cnt == 0 {
                None
            } else {
                Some(self.kth_smallest(range, cnt - 1))
            }
        }

        /// # rangeのうち、lower以上の要素で最小のもの
        pub fn next(&self, range: impl ToBounds<usize> + Clone, lower: u64) -> Option<u64> {
            let (l, r) = range.lr();
            let cnt = self.range_lower_than(range.clone(), lower);
            if cnt == r - l {
                None
            } else {
                Some(self.kth_smallest(range, cnt))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use xor_shift::XorShift;

    #[test]
    fn test_access() {
        let src = vec![5u64, 4, 5, 5, 2, 1, 5, 6, 1, 3, 5, 0];
        let wm = WaveletMatrix::from(src.clone());
        for i in 0..src.len() {
            assert_eq!(src[i], wm.access(i));
        }
        let src = vec![0u64, 0, 0, 0];
        let wm = WaveletMatrix::from(src.clone());
        for i in 0..src.len() {
            assert_eq!(src[i], wm.access(i));
        }
    }

    #[test]
    fn test_rank() {
        let src = vec![5u64, 4, 5, 5, 2, 1, 5, 6, 1, 3, 5, 0];
        let wm = WaveletMatrix::from(src.clone());
        for i in 0..10 {
            let mut cnt = 0;
            for j in 0..src.len() {
                if src[j] == i {
                    cnt += 1;
                }
                assert_eq!(cnt, wm.rank(i, j + 1), "{} {}", i, j);
            }
        }
    }

    #[test]
    fn test_select() {
        let n = 1000000;
        let mut rand = XorShift::from_time();
        let src = (0..n)
            .map(|_| rand.rand_range(0..10000000000000) as u64)
            .collect::<Vec<_>>();
        let mut map = HashMap::default();
        let wm = WaveletMatrix::from(src.clone());
        for i in 0..n {
            *map.entry(src[i]).or_insert(0) += 1;
            let c = map.get(&src[i]).unwrap();
            assert_eq!(wm.select(src[i], *c), Some(i));
        }
    }

    #[test]
    fn test_kth_smallest() {
        let n = 200;
        let mut rand = XorShift::from_time();
        let src = (0..n)
            .map(|_| rand.rand_range(0..10000000000000) as u64)
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::from(src.clone());
        for l in 0..n {
            for r in l + 1..n {
                let mut v = Vec::new();
                for i in l..r {
                    v.push(src[i]);
                }
                v.sort();
                for i in l..r {
                    assert_eq!(v[i - l], wm.kth_smallest(l..r, i - l));
                }
            }
        }
    }

    #[test]
    fn test_kth_largest() {
        let n = 200;
        let mut rand = XorShift::from_time();
        let src = (0..n)
            .map(|_| rand.rand_range(0..10000000000000) as u64)
            .collect::<Vec<_>>();
        let wm = WaveletMatrix::from(src.clone());
        for l in 0..n {
            for r in l + 1..n {
                let mut v = Vec::new();
                for i in l..r {
                    v.push(src[i]);
                }
                v.sort();
                for i in l..r {
                    assert_eq!(v[v.len() + l - 1 - i], wm.kth_largest(l..r, i - l));
                }
            }
        }
    }
}
