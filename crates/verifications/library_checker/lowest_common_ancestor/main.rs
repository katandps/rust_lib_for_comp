// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use adjacency_list::Graph;
use io_util::*;
use lowest_common_ancestor::LowestCommonAncestor;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let mut graph = Graph::new(n);
    for i in 1..n {
        graph.add_edge(i, io.v(), ());
    }
    let lca = LowestCommonAncestor::new(&graph, 0);
    for _ in 0..q {
        let (u, v) = io.v2::<usize, usize>();
        io.out(lca.query(u, v).line());
    }
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 5
        0 0 2 2
        0 1
        0 4
        1 2
        2 3
        3 4",
        "0
        0
        0
        2
        2",
    ))
}
