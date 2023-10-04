//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use io_util::*;
use string_util::*;
use wavelet_matrix::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let wm = WaveletMatrix::from(io.vec::<u64>(n));
    for _ in 0..q {
        let (l, r, x) = io.v3::<usize, usize, u64>();
        let ans = wm.rank_section(l..r, x);
        io.out(ans.line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 3
        3 7 1 2 1
        1 5 1
        3 3 0
        0 4 3",
        "2
        0
        1",
    ))
}
