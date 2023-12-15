// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_xor_convolution
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use bitwise_convolution::{convolution, xor_convolution::XorConvolution};
use io_util::*;
use mod_int::ModInt;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let a = io.vec::<ModInt<998_244_353>>(1 << n);
    let b = io.vec::<ModInt<998_244_353>>(1 << n);
    io.out(
        &convolution::<XorConvolution, 998_244_353>(a, b)
            .join(" ")
            .line(),
    );
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3
        1 2 3 4 5 6 7 8
        9 10 11 12 13 14 15 16",
        "492 488 476 472 428 424 412 408
        ",
    ));
}
