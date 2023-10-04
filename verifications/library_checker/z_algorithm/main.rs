//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default())
}
use io_util::*;
use string_util::*;
use z_algorithm::z;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let s = io.chars();
    io.out(z(&s).join(" "));
    io.flush();
}
#[test]
fn test() {
    solve(io_debug::IODebug::static_assert("abcbcba", "7 0 0 0 0 0 1"));
    solve(io_debug::IODebug::static_assert(
        "mississippi",
        "11 0 0 0 0 0 0 0 0 0 0",
    ));
    solve(io_debug::IODebug::static_assert(
        "ababacaca",
        "9 0 3 0 1 0 1 0 1",
    ));
    solve(io_debug::IODebug::static_assert("aaaaa", "5 4 3 2 1"));
}
