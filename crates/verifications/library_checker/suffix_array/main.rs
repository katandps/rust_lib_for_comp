// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use string_util::*;
use suffix_array::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let s = io.lowercase();
    let result = SuffixArray::build(&s);
    io.out(result.sa[1..].join(" ").line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert("abcbcba", "6 0 5 3 1 4 2"));
    solve(io_debug::IODebug::static_assert(
        "mississippi",
        "10 7 4 1 0 9 8 6 3 5 2",
    ));
    solve(io_debug::IODebug::static_assert(
        "ababacaca",
        "8 0 2 6 4 1 3 7 5",
    ));
    solve(io_debug::IODebug::static_assert("aaaaa", "4 3 2 1 0"))
}
