//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/number_of_substrings
//! # 部分文字列の個数
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use longest_common_prefix_array::*;
use string_util::*;
use suffix_array::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let s = io.lowercase();
    let sa = SuffixArray::build(&s);
    let lcp = LCPArray::build(&sa);
    let sum = lcp.lcp.iter().sum::<usize>();
    io.out((s.len() * (s.len() + 1) / 2 - sum).line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert("abcbcba", "21"));
    solve(io_debug::IODebug::static_assert("mississippi", "53"));
    solve(io_debug::IODebug::static_assert("ababacaca", "33"));
    solve(io_debug::IODebug::static_assert("aaaaa", "5"))
}
