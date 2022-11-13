//! # BitMatrix
//! $\Z_2$ を要素とする行列
//!
//! ## dependency
//! [bit-set](BitSet)
//!
//! ## verify
//! unverified

use crate::data_structure::bitset::BitSet;
use crate::prelude::*;

#[snippet(name = "bit-matrix", doc_hidden)]
#[derive(Clone, Debug)]
pub struct BitMatrix {
    height: usize,
    width: usize,
    val: Vec<BitSet>,
}

#[snippet(name = "bit-matrix", doc_hidden)]
impl BitMatrix {
    pub fn new(height: usize, width: usize) -> BitMatrix {
        let val = vec![BitSet::new(width); height];
        BitMatrix { height, width, val }
    }

    /// ## ガウス・ジョルダンの消去法
    /// 行基本変形を用いて行階段形に変形し、Rankを返す
    ///
    /// ## 計算量
    /// $O(N^3)$
    ///
    /// ## 備考
    /// もとのbit列同士のxorで得られる最大のbit列は、elimination後にすべてのBitSetをxorすることで得られる
    /// [ABC141F](https://atcoder.jp/contests/abc141/submissions/27476984)
    pub fn elimination(&mut self, is_extended: bool) -> usize {
        let mut rank = 0;
        for col in (0..self.width - usize::from(is_extended)).rev() {
            let mut pivot = None;
            for row in rank..self.height {
                if self.val[row][col] {
                    pivot = Some(row);
                    break;
                }
            }
            if let Some(pivot) = pivot {
                self.val.swap(pivot, rank);
                for row in 0..self.height {
                    if row != rank && self.val[row][col] {
                        self.val[row] = self.val[row].clone() ^ self.val[rank].clone();
                    }
                }
                rank += 1;
            }
        }
        rank
    }

    /// ## 連立1次方程式を解く
    ///
    /// ## 使い方
    /// 右辺の値をbで与える
    pub fn linear_equation(&mut self, b: &[bool]) -> Option<(Vec<bool>, usize)> {
        let mut m = BitMatrix::new(self.height, self.width + 1);
        (0..self.height).for_each(|i| {
            (0..self.width).for_each(|j| {
                m.val[i].set(j, self.val[i][j]);
            });
            m.val[i].set(self.width, b[i]);
        });
        let rank = self.elimination(true);

        if m.val.iter().skip(rank).filter(|bm| bm[self.width]).count() == 0 {
            Some((
                (0..self.width).map(|i| m.val[i][self.width]).collect(),
                rank,
            ))
        } else {
            None
        }
    }
}
