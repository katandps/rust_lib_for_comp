//! ウェーブレット行列
//! ## dependencies
//! [完備辞書](crate::data_structure::succinct_indexable_dictionaries)

use crate::data_structure::succinct_indexable_dictionaries::{SIDBuilder, SID};

pub struct WaveletMatrix {
    depth: usize,
    size: usize,
    matrix: Vec<SID>,
    mid: Vec<usize>,
}

///
/// ## 計算量
/// $`O(NlogN)`$
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
            size,
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

    pub fn rank() {
        todo!()
    }

    pub fn kth_smallest() {
        todo!()
    }

    pub fn kth_largest() {
        todo!()
    }

    pub fn range_freq() {
        todo!()
    }

    pub fn prev() {
        todo!()
    }

    pub fn next() {
        todo!()
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
