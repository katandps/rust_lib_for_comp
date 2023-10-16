// verification-helper: PROBLEM https://atcoder.jp/contests/abc282/tasks/abc282_d
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use adjacency_list::Graph;
use bipartite_graph::BipartiteGraphTrait;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, m) = io.v2::<usize, usize>();
    let uv = io.vec2::<usize, usize>(m);
    let mut graph = Graph::new(n);
    for (u, v) in uv {
        graph.add_edge(u - 1, v - 1, 1);
    }
    let ans = if let Some(v) = graph.bipartition() {
        let mut cnt = vec![vec![0, 0]; n];
        for (c, b) in v {
            cnt[c][usize::from(b)] += 1;
        }
        let mut ans = 0i64;
        for i in 0..n {
            ans += (cnt[i][0] + cnt[i][1]) * (n as i64 - cnt[i][0] - cnt[i][1])
                + cnt[i][0] * cnt[i][1] * 2;
        }
        ans / 2 - m as i64
    } else {
        0
    };
    io.out(ans.line());
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 4
        4 2
        3 1
        5 2
        3 2",
        "2",
    ));
    solve(io_debug::IODebug::static_assert(
        "4 3
        3 1
        3 2
        1 2",
        "0",
    ));
    solve(io_debug::IODebug::static_assert(
        "9 11
        4 9
        9 1
        8 2
        8 3
        9 2
        8 4
        6 7
        4 6
        7 5
        4 5
        7 8
        ",
        "9",
    ));
}
