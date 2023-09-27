//! <https://judge.yosupo.jp/problem/range_kth_smallest>

use io_util::*;
use string_util::*;
use wavelet_matrix::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let wm = WaveletMatrix::from(io.vec::<u64>(n));
    for _ in 0..q {
        let (l, r, k) = io.v3::<usize, usize, usize>();
        let ans = wm.kth_smallest(l..r, k);
        io.out(ans.line());
    }
    io.flush();
}
#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 3
        1 4 0 1 3
        0 5 2
        1 3 1
        3 4 0",
        "1
        4
        1",
    ))
}
