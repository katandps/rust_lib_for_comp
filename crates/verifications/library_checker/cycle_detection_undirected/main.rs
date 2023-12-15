// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cycle_detection_undirected
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use adjacency_list::Graph;
use find_cycle::*;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, m) = io.v2::<usize, usize>();
    let uv = io.vec2::<usize, usize>(m);
    let mut graph = Graph::new(n);
    for &(u, v) in &uv {
        graph.add_edge(u, v, ());
    }
    let cycle = graph.find_cycle(true);
    if let Some(v) = cycle {
        io.out(v.len().line());
        let mut edges = Vec::new();
        let mut vertices = Vec::new();
        for i in v {
            edges.push(i / 2);
            vertices.push(graph.edges[i].0);
        }

        io.out(vertices.join(" ").line());
        io.out(edges.join(" ").line());
    } else {
        io.out((-1).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "6 6
        0 2
        0 3
        4 2
        3 1
        2 1
        2 5",
        "4
        3 0 2 1
        1 0 4 3",
    ));
    solve(io_debug::IODebug::static_assert(
        "10 1
        3 3",
        "1
        3
        0",
    ));
    solve(io_debug::IODebug::static_assert(
        "10 3
        3 5
        3 5
        5 3",
        "2
        5 3
        1 0",
    ));
    solve(io_debug::IODebug::static_assert(
        "6 5
        0 3
        2 0
        1 3
        3 5
        4 2",
        "-1",
    ));
    solve(io_debug::IODebug::static_assert("6 0", "-1"));
}
