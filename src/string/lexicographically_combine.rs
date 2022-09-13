//! # 文字列の辞書順最小の結合
//! 文字列(Vec<char>)の配列を入力にとる。
//! 文字列同士を自由に並び替えて結合したとき、辞書順最小になるものを得る
//!
//! ## 概要
//! $a + b < b + a$ ならば a + bの順で結合してよい
//!
//! ## verify
//! [ABC042_B](https://atcoder.jp/contests/abc042/submissions/34830454)

use crate::prelude::*;

#[snippet(name = "lexicographically-combine", doc_hidden)]
pub fn lexicographically_combine(mut s: Vec<Vec<char>>) -> Vec<char> {
    s.sort_by(|a, b| {
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
    s.into_iter().flatten().collect::<Vec<_>>()
}
