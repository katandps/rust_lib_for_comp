//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency
use io_util::*;
use string_util::*;
use wavelet_matrix::WaveletMatrix;

fn main() {
    let mut io = IO::default();
    let (n, q) = io.v2::<usize, usize>();
    let wm = WaveletMatrix::from(io.vec::<u64>(n));
    for _ in 0..q {
        let (l, r, x) = io.v3::<usize, usize, u64>();
        let ans = wm.rank_range(x, &(l..r));
        io.out(ans.ln());
    }
    io.flush();
}
