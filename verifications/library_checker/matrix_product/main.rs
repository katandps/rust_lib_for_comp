// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_product
//! # 行列積
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use io_util::*;
use matrix::Matrix;
use mod_int::mod998244353::Mi;
use prelude::TryFrom;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, m, k) = io.v3::<usize, usize, usize>();
    let a = io.matrix::<Mi>(n, m);
    let b = io.matrix::<Mi>(m, k);
    let am = Matrix::try_from(a).unwrap();
    let bm = Matrix::try_from(b).unwrap();
    let c = (am * bm).unwrap();
    io.out(c.line());
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "2 2 2
        1 1
        1 0
        5 2
        3 1",
        "8 3
        5 2",
    ));
    solve(io_debug::IODebug::static_assert(
        "1 2 3
        1 2
        3 4 5
        6 7 8",
        "15 18 21",
    ));
    solve(io_debug::IODebug::static_assert(
        "1 1 1
        123456
        789012",
        "578563231",
    ))
}
