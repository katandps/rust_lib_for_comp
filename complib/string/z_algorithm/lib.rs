//! # Z-algorithm
//! 文字列SとSのi文字目から始まる文字列の共通部分列の長さを求める
//! ## 計算量
//! $O(N)$

use crate::prelude::*;

#[codesnip::entry("z-algorithm", doc_hidden)]
pub fn z(src: &[char]) -> Vec<usize> {
    let mut c = 0;
    let n = src.len();
    let mut ret = vec![0; n];
    for i in 1..n {
        let l = i - c;
        if i + ret[l] < c + ret[c] {
            ret[i] = ret[l];
        } else {
            let mut j = if c + ret[c] > i { c + ret[c] - i } else { 0 };
            while i + j < n && src[j] == src[i + j] {
                j += 1;
            }
            ret[i] = j;
            c = i;
        }
    }
    ret[0] = n;
    ret
}
