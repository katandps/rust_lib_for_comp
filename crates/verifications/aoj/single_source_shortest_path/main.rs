// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_A
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use adjacency_list::Graph;
use dijkstra::*;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (v, e, r) = io.v3::<usize, usize, usize>();
    let std = io.vec3::<usize, usize, i64>(e);
    let mut graph = Graph::new(v);
    for (s, t, d) in std {
        graph.add_arc(s, t, d);
    }
    let d = Dijkstra::calc(&graph, r);
    for i in 0..v {
        if d.dist[i] == i64::MAX {
            io.out("INF".line());
        } else {
            io.out(d.dist[i].line())
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4 5 0
        0 1 1
        0 2 4
        1 2 2
        2 3 1
        1 3 5",
        "0
        1
        3
        4
        ",
    ));
    solve(io_debug::IODebug::static_assert(
        "4 6 1
        0 1 1
        0 2 4
        2 0 1
        1 2 2
        3 1 1
        3 2 5",
        "3
        0
        2
        INF",
    ))
}
