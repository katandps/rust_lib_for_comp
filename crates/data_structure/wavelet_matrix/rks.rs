//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest
use io_util::*;
use string_util::*;
use wavelet_matrix::WaveletMatrix;

fn main() {
    let mut io = IO::default();
    let (n, q) = io.v2::<usize, usize>();
    let wm = WaveletMatrix::from(io.vec::<u64>(n));
    for _ in 0..q {
        let (l, r, k) = io.v3::<usize, usize, usize>();
        let ans = wm.kth_smallest(&(l..r), k);
        io.out(ans.ln());
    }
    io.flush();
}
