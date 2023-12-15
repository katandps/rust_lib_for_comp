// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorial
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use embedded_mod_factorial::Factorial;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v() {
        let n = io.v::<i64>();
        io.out(Factorial::factorial(n).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5
        0
        5
        100
        1234567
        998244352",
        "1
        120
        35305197
        972177311
        998244352",
    ))
}
