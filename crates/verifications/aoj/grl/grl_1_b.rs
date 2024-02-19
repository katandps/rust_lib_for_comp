//! # Single Source Shortest Path (Negative Edges)(単一始点最短経路（負の重みをもつ辺を含む）)

use rust_lib_for_comp::graph::adjacency_list::Graph;
use rust_lib_for_comp::graph::bellman_ford::bellman_ford;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Grl1B;
impl verify::Solver for Grl1B {
    const PROBLEM_ID: &'static str = "GRL_1_B";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (v, e, r) = reader.v3::<usize, usize, usize>();
        let std = reader.vec3::<usize, usize, i64>(e);
        let mut graph = Graph::new(v);
        for (s, t, d) in std {
            graph.add_arc(s, t, d);
        }
        let d = bellman_ford(&graph, r);
        for di in &d {
            if di == &std::i64::MIN {
                writeln!(write, "NEGATIVE CYCLE").ok();
                write.flush().ok();
                return;
            }
        }
        for di in d {
            if di == i64::MAX {
                writeln!(write, "INF").ok();
            } else {
                writeln!(write, "{di}").ok();
            }
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Grl1B::assert(
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
    );
    Grl1B::assert(
        "4 6 0
    0 1 2
    0 2 3
    1 2 -5
    1 3 1
    2 3 2
    3 1 0",
        "NEGATIVE CYCLE",
    );
    Grl1B::assert(
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
    )
}
