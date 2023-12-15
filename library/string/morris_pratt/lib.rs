//! # MP法(モリス-プラット法)
//!
//! 文字列Sが与えられる。
//! 先頭から各iまでの文字列について、接頭辞と接尾辞が何文字一致しているかをborderと呼ぶ。
//! この値をならし計算量$O(N)$ですべて求める。
//!
use crate::prelude::*;

#[codesnip::entry("morris-pratt", doc_hidden)]
pub fn border<T: PartialEq>(src: &[T]) -> Vec<i64> {
    let mut ret = vec![0; src.len() + 1];
    ret[0] = -1;
    let mut j = -1;
    for i in 0..src.len() {
        while j >= 0 && src[i] != src[j as usize] {
            j = ret[j as usize]
        }
        j += 1;
        ret[i + 1] = j;
    }
    ret
}

#[test]
fn test_mp() {
    let src = "aabaabaaa".chars().collect::<Vec<_>>();
    let ret = border(&src);
    assert_eq!(ret, vec![-1, 0, 1, 0, 1, 2, 3, 4, 5, 2]);
}
