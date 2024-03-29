//! # Single Source Shortest Path(単一始点最短経路)

use rust_lib_for_comp::graph::adjacency_list::Graph;
use rust_lib_for_comp::graph::dijkstra::Dijkstra;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Grl1A;
impl verify::Solver for Grl1A {
    const PROBLEM_ID: &'static str = "GRL_1_A";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (v, e, r) = reader.v3::<usize, usize, usize>();
        let std = reader.vec3::<usize, usize, i64>(e);
        let mut graph = Graph::new(v);
        for (s, t, d) in std {
            graph.add_arc(s, t, d);
        }
        let d = Dijkstra::calc(&graph, r);
        for i in 0..v {
            if d.dist[i] == i64::MAX {
                writeln!(write, "INF").ok();
            } else {
                writeln!(write, "{}", d.dist[i]).ok();
            }
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Grl1A::assert(
        "4 5 0
    0 1 1
    0 2 4
    1 2 2
    2 3 1
    1 3 5",
        "0
        1
        3
        4",
    );
    Grl1A::assert(
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
    )
}
