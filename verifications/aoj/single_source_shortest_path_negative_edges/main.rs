// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_B
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use adjacency_list::Graph;
use bellman_ford::bellman_ford;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (v, e, r) = io.v3::<usize, usize, usize>();
    let std = io.vec3::<usize, usize, i64>(e);
    let mut graph = Graph::new(v);
    for (s, t, d) in std {
        graph.add_arc(s, t, d);
    }
    let d = bellman_ford(&graph, r);
    for di in &d {
        if di == &std::i64::MIN {
            io.out("NEGATIVE CYCLE".line());
            return io.flush();
        }
    }
    for di in d {
        if di == i64::MAX {
            io.out("INF".line());
        } else {
            io.out(di.line())
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4 5 0
        0 1 2
        0 2 3
        1 2 -5
        1 3 1
        2 3 2",
        "0
        2
        -3
        -1",
    ));
    solve(io_debug::IODebug::static_assert(
        "4 6 0
        0 1 2
        0 2 3
        1 2 -5
        1 3 1
        2 3 2
        3 1 0",
        "NEGATIVE CYCLE",
    ));
    solve(io_debug::IODebug::static_assert(
        "4 5 1
    0 1 2
    0 2 3
    1 2 -5
    1 3 1
    2 3 2",
        "INF
        0
        -5
        -3",
    ))
}
