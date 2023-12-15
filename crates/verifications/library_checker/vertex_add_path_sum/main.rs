// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default())
}
use addition::Addition;
use adjacency_list::Graph;
use heavy_light_decomposition::HLDecomposition;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let mut graph = Graph::new(n);
    for (u, v) in io.vec2::<usize, usize>(n - 1) {
        graph.add_edge(u, v, ());
    }
    let mut hld = HLDecomposition::<Addition<i64>>::build(&graph, 0, &a);
    for _ in 0..q {
        if 0 == io.v() {
            let (p, x) = io.v2::<usize, i64>();
            hld.update_at(p, hld.prod_path(p, p) + x);
        } else {
            let (u, v) = io.v2::<usize, usize>();
            io.out(hld.prod_path(u, v).line());
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 5
        1 10 100 1000 10000
        0 1
        1 2
        2 3
        1 4
        1 0 3
        1 2 4
        0 1 100000
        1 1 3
        1 3 2",
        "1111
        10110
        101110
        1100",
    ))
}
