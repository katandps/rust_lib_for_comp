// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_det
//! # 行列式
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use std::convert::TryFrom;

use io_util::*;
use matrix::{Determinant, Matrix};
use mod_int::mod998244353::Mi;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let a = io.matrix::<Mi>(n, n);
    let matrix = Matrix::try_from(a).unwrap();
    io.out(matrix.determinant().unwrap().line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3
        3 1 4
        1 5 9
        2 6 5",
        "998244263",
    ));
    solve(io_debug::IODebug::static_assert(
        "3
        1 2 3
        4 5 6
        7 8 9",
        "0",
    ));
    solve(io_debug::IODebug::static_assert(
        "2
        0 1
        1 0",
        "998244352",
    ))
}