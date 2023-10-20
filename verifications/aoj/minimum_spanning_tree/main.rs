// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/2/GRL_2_A
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use adjacency_list::Graph;
use io_util::*;
use kruskal::Kruskal;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (v, e) = io.v2::<usize, usize>();
    let std = io.vec3::<usize, usize, i64>(e);
    let mut graph = Graph::new(v);
    for (s, t, d) in std {
        graph.add_edge(s, t, d);
    }
    let d = Kruskal::from(&graph);
    io.out(d.sum().line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "6 9
        0 1 1
        0 2 3
        1 2 1
        1 3 7
        2 4 1
        1 4 3
        3 4 1
        3 5 1
        4 5 6",
        "5",
    ));
}
