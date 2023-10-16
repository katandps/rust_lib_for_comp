// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cycle_detection
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
    for (u, v) in uv {
        graph.add_arc(u, v, ());
    }
    let cycle = graph.find_cycle(false);
    if let Some(v) = cycle {
        io.out(v.len().line());
        io.out(v.join("\n").line());
    } else {
        io.out((-1).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 7
        0 3
        0 4
        4 2
        4 3
        4 0
        2 1
        1 0",
        "4
        6 1 2 5",
    ));
    solve(io_debug::IODebug::static_assert(
        "2 1
        1 0",
        "-1",
    ));
    solve(io_debug::IODebug::static_assert(
        "4 6
        0 1
        1 2
        2 0
        0 1
        1 3
        3 0",
        "3
        2 0 1",
    ));
}
