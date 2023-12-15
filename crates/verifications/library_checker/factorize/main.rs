// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use pollard_rho::PollardRho;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    for _ in 0..n {
        let p = io.v::<u64>().prime_factorize();
        io.out(format!("{} {}\n", p.len(), p.join(" ")));
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "10
        1
        2
        3
        4
        5
        6
        7
        8
        9
        10",
        "0
        1 2
        1 3
        2 2 2
        1 5
        2 2 3
        1 7
        3 2 2 2
        2 3 3
        2 2 5
        ",
    ))
}
