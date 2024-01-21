// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use miller_rabin::MillerRabin;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v::<usize>() {
        let n = io.v::<u64>();
        io.out(if n.is_prime() { "Yes" } else { "No" }.line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "6
        1
        2
        3
        4
        998244353
        1000000000000000000",
        "No
        Yes
        Yes
        No
        Yes
        No",
    ));
}
