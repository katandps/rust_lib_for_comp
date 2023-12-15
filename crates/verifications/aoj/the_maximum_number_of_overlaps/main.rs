// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DSL_5_B
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use addition::Addition;
use binary_indexed_tree_2d::BinaryIndexedTree2;
use io_util::*;
use min_max_macro::{chmax, max};
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let lr = io.vec4::<usize, usize, usize, usize>(n);
    let mut bit2d = BinaryIndexedTree2::<Addition<i64>>::new(1010, 1010);
    for (lx, ly, rx, ry) in lr {
        bit2d.add(lx + 1, ly + 1, 1);
        bit2d.add(lx + 1, ry + 1, -1);
        bit2d.add(rx + 1, ly + 1, -1);
        bit2d.add(rx + 1, ry + 1, 1);
    }
    let mut ans = 0;
    for i in 0..1010 {
        for j in 0..1010 {
            chmax!(ans, bit2d.sum(i, j));
        }
    }
    io.out(ans.line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "2
        0 0 3 2
        2 1 4 3",
        "2",
    ));
    solve(io_debug::IODebug::fvalue_assert(
        "2
        0 0 2 2
        2 0 4 2",
        "1",
    ));
    solve(io_debug::IODebug::fvalue_assert(
        "3
        0 0 2 2
        0 0 2 2
        0 0 2 2",
        "3",
    ))
}
