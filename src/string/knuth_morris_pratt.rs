//! # KMP法(クヌース・モリス・プラット法)
//!
//! 文字列Sが与えられる。
//! 先頭から各iまでの文字列について、接頭辞と接尾辞が何文字一致しているかをborderと呼ぶ。
//! 接頭辞Pのtagged borderとは、Pのborder Wのうち、P[|W|] != 残りの文字列\[0\] であるもの
//! この値をならし計算量$O(N)$ですべて求める。
//!
//! ## verify
//! [ABC150F](https://atcoder.jp/contests/abc150/submissions/34853947)
use crate::prelude::*;

#[snippet(name = "knuth-morris-pratt", doc_hidden)]
pub fn tagged_border<T: PartialEq>(src: &[T]) -> Vec<i64> {
    let n = src.len();
    let (mut ret, mut j) = (vec![0; n + 1], -1);
    ret[0] = -1;
    for i in 0..n {
        while j >= 0 && src[i] != src[j as usize] {
            j = ret[j as usize]
        }
        j += 1;
        ret[i + 1] = j;
    }
    ret
}

#[snippet(name = "knuth-morris-pratt", doc_hidden)]
pub fn search<T: PartialEq>(src: &[T], word: &[T]) -> Vec<i64> {
    let table = tagged_border(word);
    let (mut m, mut i, n, mut ret) = (0, 0, src.len() as i64, Vec::new());
    while m + i < n {
        if word[i as usize] == src[(m + i) as usize] {
            i += 1;
            if i == word.len() as i64 {
                ret.push(m);
                m = m + i - table[i as usize];
                i = table[i as usize];
            }
        } else {
            m = m + i - table[i as usize];
            if i > 0 {
                i = table[i as usize]
            }
        }
    }
    ret
}

#[test]
fn test_kmp() {
    let src = "ABCDEABCABAAABC".chars().collect::<Vec<_>>();
    let word = "ABC".chars().collect::<Vec<_>>();
    assert_eq!(vec![0, 5, 12], search(&src, &word));
}
