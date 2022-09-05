//! # KMP法(クヌース・モリス・プラット法)
//!
//! 文字列Sが与えられる。
//! 先頭から各iまでの文字列について、接頭辞と接尾辞が何文字一致しているかをborderと呼ぶ。
//! 接頭辞Pのtagged borderとは、Pのborder Wのうち、P[|W|] != 残りの文字列[0] であるもの
//! この値をならし計算量$O(N)$ですべて求める。
//!
use crate::prelude::*;

#[snippet(name = "knuth-morris-pratt", doc_hidden)]
pub fn tagged_border<T: PartialEq>(src: &[T]) -> Vec<i64> {
    let mut ret = vec![0; src.len() + 1];
    ret[0] = -1;
    let mut j = -1;
    for i in 0..src.len() {
        while j >= 0 && src[i] != src[j as usize] {
            j = ret[j as usize]
        }
        j += 1;
        if i + 1 < src.len() && src[i + 1] == src[j as usize] {
            ret[i + 1] = ret[j as usize]
        } else {
            ret[i + 1] = j;
        }
    }
    ret
}

pub fn search<T: PartialEq>(_src: &[T], _word: &[T]) {
    todo!()
}

#[test]
fn test_kmp() {
    let src = "GCAGAGAG".chars().collect::<Vec<_>>();
    let ret = tagged_border(&src);
    assert_eq!(ret, vec![-1, 0, 0, -1, 1, -1, 1, -1, 1]);
}
