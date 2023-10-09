// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use bitwise_convolution::{and_convolution::AndConvolution, convolution};
use io_util::*;
use mod_int::mod998244353::Mod998_244_353;
use mod_int::ModInt;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let a = io.vec::<ModInt<Mod998_244_353>>(1 << n);
    let b = io.vec::<ModInt<Mod998_244_353>>(1 << n);
    io.out(&convolution::<_, AndConvolution>(a, b).join(" ").line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3
        1 2 3 4 5 6 7 8
        9 10 11 12 13 14 15 16",
        "957 412 515 208 751 292 337 128",
    ));
}
