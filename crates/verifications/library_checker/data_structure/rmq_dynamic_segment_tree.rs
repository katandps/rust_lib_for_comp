// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use dynamic_segment_tree::DynamicSegmentTree;
use io_util::*;
use minimization::Minimization;
use range_traits::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let mut segtree = DynamicSegmentTree::<Minimization<i64>>::new(500010);
    for (i, &ai) in a.iter().enumerate() {
        segtree.set(i as i64, ai);
    }
    for _ in 0..q {
        let (l, r) = io.v2::<i64, i64>();
        io.out(segtree.product(l..r).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4 10
        2 10 1 100
        0 1
        0 2
        0 3
        0 4
        1 2
        1 3
        1 4
        2 3
        2 4
        3 4",
        "2
        2
        1
        1
        10
        1
        1
        1
        1
        100",
    ))
}
