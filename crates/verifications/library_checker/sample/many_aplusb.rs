// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v::<usize>() {
        let (a, b) = io.v2::<i64, i64>();
        io.out((a + b).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3
        1 2
        11 22
        1000000000000000000 1000000000000000000",
        "3
        33
        2000000000000000000",
    ));
}
