//! # ウェーブレット行列
//! ## 概要
//! 正整数列について、構築$`O(NlogV)`$で$`O(logV)`$ or $`O(log^2V)`$のクエリを提供する
//!
//! ## dependencies
//! [完備辞書](crate::data_structure::succinct_indexable_dictionaries)
//!
//! ## 計算量
//! - 構築: $`O(NlogV)`$
//! - クエリ: $`O(log^2V)`$
//!
//! ## verify
//! unverified
use crate::data_structure::succinct_indexable_dictionaries::{SIDBuilder, SID};
use crate::*;

pub struct WaveletMatrix {
    depth: usize,
    _size: usize,
    matrix: Vec<SID>,
    mid: Vec<usize>,
}

impl From<Vec<u64>> for WaveletMatrix {
    fn from(mut src: Vec<u64>) -> Self {
        let size = src.len();
        let depth = 64;
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

        Self {
            _size: size,
            depth,
            matrix,
            mid,
        }
    }
}

impl WaveletMatrix {
    ///
    /// ## 計算量
    /// $`O(logN)`$
    pub fn access(&self, mut k: usize) -> u64 {
        let mut ret = 0;
        (0..self.depth).rev().for_each(|level| {
            let f = self.matrix[level].access(k);
            if f {
                ret |= 1u64 << level
            }
            k = self.matrix[level].rank(k, f) as usize + self.mid[level] * if f { 1 } else { 0 };
        });
        ret
    }

    fn succ(&self, b: bool, l: usize, r: usize, level: usize) -> (usize, usize) {
        (
            self.matrix[level].rank(l, b) + self.mid[level] * if b { 1 } else { 0 },
            self.matrix[level].rank(r, b) + self.mid[level] * if b { 1 } else { 0 },
        )
    }

    /// $`[0 <= i < r) かつ v[i] == x`$ であるようなiの個数
    pub fn rank(&self, x: u64, r: usize) -> usize {
        let (_l, r) = (0..self.depth).rev().fold((0, r), |(l, r), level| {
            self.succ((x >> level) & 1 > 0, l, r, level)
        });
        r - 1
    }

    /// range のうち、小さい方からk番目の数
    pub fn kth_smallest<R: RangeBounds<usize>>(&self, range: &R, mut k: usize) -> u64 {
        let (l, r) = to_lr(range, std::usize::MAX);
        assert!(k < r - l);
        let mut ret = 0;
        (0..self.depth).rev().fold((l, r), |(l, r), level| {
            let cnt = self.matrix[level].rank(r, false) - self.matrix[level].rank(l, false);
            if cnt <= k {
                ret |= 1 << level;
                k -= cnt;
            }
            self.succ(cnt <= k, l, r, level)
        });
        ret
    }
    /// range のうち、大きい方からk番目の数
    pub fn kth_largest<R: RangeBounds<usize>>(&self, range: &R, k: usize) -> u64 {
        let (l, r) = to_lr(range, std::usize::MAX);
        self.kth_smallest(range, r - l - k - 1)
    }

    /// range のうち、upper未満のものの個数
    pub fn range_freq<R: RangeBounds<usize>>(&self, range: &R, upper: u64) -> usize {
        let (l, r) = to_lr(range, std::usize::MAX);
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

    /// rangeのうち、 upperより小さい要素で最大のもの
    pub fn prev<R: RangeBounds<usize>>(&self, range: &R, upper: u64) -> Option<u64> {
        let cnt = self.range_freq(range, upper);
        if cnt == 0 {
            None
        } else {
            Some(self.kth_smallest(range, cnt - 1))
        }
    }

    /// rangeのうち、lower以上の要素で最小のもの
    pub fn next<R: RangeBounds<usize>>(&self, range: &R, lower: u64) -> Option<u64> {
        let (l, r) = to_lr(range, std::usize::MAX);
        let cnt = self.range_freq(range, lower);
        if cnt == r - l {
            None
        } else {
            Some(self.kth_smallest(range, cnt))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let src = vec![5, 7, 3, 4, 2, 9];
        let wm = WaveletMatrix::from(src.clone());
        src.iter()
            .enumerate()
            .for_each(|(i, &src)| assert_eq!(src, wm.access(i)));
    }
}
