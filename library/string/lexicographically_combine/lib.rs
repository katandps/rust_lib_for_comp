//! # 文字列の辞書順最小の結合
//! 文字列のスライスを入力にとる。
//! 文字列同士を自由に並び替えて結合したとき、辞書順最小になるものを得る
//!
//! ## 概要
//! $a + b < b + a$ ならば a + bの順で結合してよい
//!
//! ## verify
//! [ABC042_B](https://atcoder.jp/contests/abc042/submissions/34830454)

use crate::prelude::*;

#[codesnip::entry("lexicographically-combine", doc_hidden)]
pub fn lexicographically_combine(s: &[Vec<char>]) -> Vec<char> {
    let mut v = (0..s.len()).collect::<Vec<_>>();
    v.sort_by(|&ai, &bi| {
        let (a, b) = (&s[ai], &s[bi]);
        let mut s = Vec::new();
        let mut t = Vec::new();
        for &ai in a {
            s.push(ai);
        }
        for &bi in b {
            s.push(bi);
            t.push(bi);
        }
        for &ai in a {
            t.push(ai);
        }
        s.cmp(&t)
    });
    v.into_iter().flat_map(|i| s[i].clone()).collect::<Vec<_>>()
}
