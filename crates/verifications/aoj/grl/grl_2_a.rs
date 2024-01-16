//! # Minimum Spanning Tree(最小全域木)

use rust_lib_for_comp::graph::adjacency_list::Graph;
use rust_lib_for_comp::graph::kruskal::Kruskal;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Grl2A;
impl verify::Solver for Grl2A {
    const PROBLEM_ID: &'static str = "GRL_2_A";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (v, e) = reader.v2::<usize, usize>();
        let std = reader.vec3::<usize, usize, i64>(e);
        let mut graph = Graph::new(v);
        for (s, t, d) in std {
            graph.add_edge(s, t, d);
        }
        let d = Kruskal::from(&graph);
        writeln!(write, "{}", d.sum()).ok();
        write.flush().ok();
    }
}
