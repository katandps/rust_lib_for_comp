//! # All Pairs Shortest Path

use rust_lib_for_comp::graph::adjacency_list::Graph;
use rust_lib_for_comp::graph::warshall_floyd::WarshallFloyd;
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::util::string_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Grl1C;
impl verify::Solver for Grl1C {
    const PROBLEM_ID: &'static str = "GRL_1_C";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (v, e) = reader.v2::<usize, usize>();
        let mut graph = Graph::new(v);
        for _ in 0..e {
            let (s, t, d) = reader.v3::<usize, usize, i64>();
            graph.add_arc(s, t, d);
        }
        let wf = WarshallFloyd::build(&graph);
        if wf.contains_negative_cycle() {
            writeln!(write, "NEGATIVE CYCLE").ok();
        } else {
            for i in 0..v {
                writeln!(
                    write,
                    "{}",
                    (0..v)
                        .map(|j| {
                            let d = wf.dist(i, j);
                            if d == i64::MAX {
                                "INF".to_string()
                            } else {
                                d.to_string()
                            }
                        })
                        .join(" "),
                )
                .ok();
            }
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    Grl1C::assert(
        "4 6
    0 1 1
    0 2 5
    1 2 2
    1 3 4
    2 3 1
    3 2 7",
        "0 1 3 4
    INF 0 2 3
    INF INF 0 1
    INF INF 7 0",
    );
    Grl1C::assert(
        "4 6
    0 1 1
    0 2 -5
    1 2 2
    1 3 4
    2 3 1
    3 2 7",
        "0 1 -5 -4
    INF 0 2 3
    INF INF 0 1
    INF INF 7 0",
    );
    Grl1C::assert(
        "4 6
    0 1 1
    0 2 5
    1 2 2
    1 3 4
    2 3 1
    3 2 -7",
        "NEGATIVE CYCLE",
    )
}
