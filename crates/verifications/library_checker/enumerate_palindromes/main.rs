// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use manachar::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let s = io.chars();
    let result = Manachar::manachar(&s);
    io.out(result.join(" ").line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "abcbcba",
        "1 0 1 0 3 0 7 0 3 0 1 0 1",
    ));
}
