// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/1549
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use io_util::*;
use string_util::*;
use wavelet_matrix::WaveletMatrix;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let a = io.vec::<u64>(n);
    let wm = WaveletMatrix::from(a);
    for _ in 0..io.v() {
        let (l, r, d) = io.v3::<usize, usize, u64>();
        let prev = wm.prev(l..=r, d);
        let next = wm.next(l..=r, d);
        let ans = match (prev, next) {
            (Some(prev), Some(next)) => std::cmp::min(d - prev, next - d),
            (Some(prev), _) => d - prev,
            (_, Some(next)) => next - d,
            _ => 0,
        };
        io.out(ans.line())
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3
        1 2 3
        3
        0 2 2
        0 2 4
        0 0 2",
        "0
        1
        1",
    ))
}
