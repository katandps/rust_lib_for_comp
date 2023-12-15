// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v::<usize>() {
        let (a, b) = io.v2::<i128, i128>();
        io.out((a + b).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5
        1 2
        11 22
        -111 -222
        10000000000000000000000000000000000000 10000000000000000000000000000000000000
        1234567890123456789012345678901234567 -10000000000000000000000000000000000000",
        "3
        33
        -333
        20000000000000000000000000000000000000
        -8765432109876543210987654321098765433",
    ));
}
