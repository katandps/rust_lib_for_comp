// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use affine::{Affine, Composition};
use dual_segment_tree::DualSegmentTree;
use io_util::*;
use mod_int::mod998244353::Mi;
use range_traits::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<Mi>(n);
    let mut segtree = DualSegmentTree::new(&a, Composition::new());
    for _ in 0..q {
        if 0 == io.v() {
            let (l, r, b, c) = io.v4::<usize, usize, Mi, Mi>();
            segtree.update_range(l..r, Affine::new(b, c));
        } else {
            let i = io.v::<usize>();
            io.out(segtree.get(i).line());
        }
    }
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 12
        1 2 3 4 5
        0 2 4 100 101
        1 0
        1 1
        1 2
        1 3
        1 4
        0 1 3 102 103
        1 0
        1 1
        1 2
        1 3
        1 4",
        "1
        2
        401
        501
        5
        1
        307
        41005
        501
        5
        ",
    ))
}
