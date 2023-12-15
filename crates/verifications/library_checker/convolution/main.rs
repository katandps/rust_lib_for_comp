// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use fast_fourier_transform::FFT;
use io_util::*;
use mod_int::ModInt;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, m) = io.v2::<usize, usize>();
    let a = io.vec::<ModInt<998_244_353>>(n);
    let b = io.vec::<ModInt<998_244_353>>(m);
    let fft = FFT::setup();
    let result = fft.convolution(a, b);
    io.out(result.join(" ").line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4 5
        1 2 3 4
        5 6 7 8 9",
        "5 16 34 60 70 70 59 36",
    ));
    solve(io_debug::IODebug::static_assert(
        "1 1
        10000000
        10000000",
        "871938225",
    ))
}
