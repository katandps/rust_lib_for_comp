// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use addition::Addition;
use binary_indexed_tree::BinaryIndexedTree;
use io_util::*;
use range_traits::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let mut bit = BinaryIndexedTree::<Addition<i64>>::from(a);
    for _ in 0..q {
        if 0 == io.v() {
            let (p, x) = io.v2::<usize, i64>();
            bit.add(p, x);
        } else {
            let (l, r) = io.v2::<usize, usize>();
            io.out(bit.product(l..r).line());
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 5
        1 2 3 4 5
        1 0 5
        1 2 4
        0 3 10
        1 0 5
        1 0 3",
        "15
        7
        25
        6",
    ))
}
