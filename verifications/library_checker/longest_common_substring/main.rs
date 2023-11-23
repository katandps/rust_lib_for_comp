// verification-helper: PROBLEM https://judge.yosupo.jp/problem/longest_common_substring
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use longest_common_prefix_array::LCPArray;
use min_max_macro::{chmax, max};
use prelude::*;
use string_util::*;
use suffix_array::SuffixArray;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let mut s = io.chars();
    let sn = s.len();
    let mut t = io.chars();
    s.push('#');
    s.append(&mut t);
    let sa = SuffixArray::build(&s);
    let lcp = LCPArray::build(&sa);
    let (mut max_size, mut sl, mut sr, mut tl, mut tr) = (0, 0, 0, 0, 0);
    // saの先頭は番兵(空文字)
    for i in 0..s.len() {
        let (mut i1, mut i2) = (sa[i], sa[i + 1]);
        if i1 > i2 {
            swap(&mut i1, &mut i2);
        }

        if i1 < sn && sn < i2 && chmax!(max_size, lcp[i + 1]) {
            let (a, b) = (i1, i2 - sn - 1);
            (sl, sr, tl, tr) = (a, a + max_size, b, b + max_size)
        }
    }
    io.out(format!("{} {} {} {}", sl, sr, tl, tr).line());
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "abcdef
        abcxdef",
        "0 3 0 3",
    ));
    solve(io_debug::IODebug::static_assert(
        "aaa
    bbbb",
        "0 0 0 0",
    ));
    solve(io_debug::IODebug::static_assert(
        "abcabcabc
        cabcabcab",
        "0 8 1 9",
    ));
    solve(io_debug::IODebug::static_assert(
        "aaa
        aaaaa",
        "0 3 2 5",
    ));
}
