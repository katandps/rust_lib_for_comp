// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default())
}
use adjacency_list::Graph;
use affine::{Affine, Composition};
use heavy_light_decomposition::HLDecomposition;
use io_util::*;
use mod_int::ModInt;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let af = io
        .vec2::<ModInt, ModInt>(n)
        .into_iter()
        .map(|(a, b)| Affine::new(a, b))
        .collect::<Vec<_>>();
    let mut graph = Graph::new(n);
    for (u, v) in io.vec2::<usize, usize>(n - 1) {
        graph.add_edge(u, v, ());
    }
    let mut hld = HLDecomposition::<Composition<ModInt<998_244_353>>>::build(&graph, 0, &af);
    for _ in 0..q {
        if 0 == io.v() {
            let (p, c, d) = io.v3::<usize, ModInt, ModInt>();
            hld.update_at(p, Affine::new(c, d));
        } else {
            let (u, v, x) = io.v3::<usize, usize, ModInt>();
            let af = hld.prod_path(u, v);
            io.out(af.apply(x).line());
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert("1 1
    100000 100000
    1 0 0 100000", ""));
    solve(io_debug::IODebug::static_assert(
        "5 5
        1 2
        3 4
        5 6
        7 8
        9 10
        0 1
        1 2
        2 3
        2 4
        1 0 3 11
        1 2 4 12
        0 2 13 14
        1 0 4 15
        1 2 2 16",
        "1555
        604
        6571
        222",
    ));
    solve(io_debug::IODebug::static_assert(
        "7 7
        1 2
        2 3
        3 4
        4 5
        5 6
        6 7
        7 8
        0 1
        1 2
        0 3
        3 4
        0 5
        5 6
        1 2 4 1
        1 4 6 1
        1 6 2 1
        0 1 20 30
        1 2 4 1
        1 4 6 1
        1 6 2 1",
        "411
        2199
        607
        3471
        2199
        6034",
    ));
}
