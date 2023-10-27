// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use affine::{Affine, Composition};
use io_util::*;
use mod_int::ModInt;
use range_traits::*;
use segment_tree::SegmentTree;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let ab = io
        .vec2::<ModInt<998_244_353>, ModInt>(n)
        .into_iter()
        .map(|(a, b)| Affine::new(a, b))
        .collect::<Vec<_>>();
    let mut segtree = SegmentTree::<Composition<ModInt>>::from(ab);
    for _ in 0..q {
        if 0 == io.v() {
            let (p, c, d) = io.v3::<usize, ModInt, ModInt>();
            segtree.update_at(p, Affine::new(c, d));
        } else {
            let (l, r, x) = io.v3::<usize, usize, ModInt>();
            io.out(segtree.product(l..r).apply(x).line());
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 5
        1 2
        3 4
        5 6
        7 8
        9 10
        1 0 5 11
        1 2 4 12
        0 1 13 14
        1 0 4 15
        1 2 5 16",
        "14005
        470
        8275
        5500",
    ))
}
