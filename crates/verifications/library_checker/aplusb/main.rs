// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (a, b) = io.v2::<i64, i64>();
    io.out((a + b).line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert("1234 5678", "6912"));
    solve(io_debug::IODebug::static_assert(
        "1000000000 1000000000",
        "2000000000",
    ));
}
