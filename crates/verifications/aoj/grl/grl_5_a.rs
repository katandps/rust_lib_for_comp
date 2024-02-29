//! # Diameter of a Tree(木の直径)

use rust_lib_for_comp::graph::{adjacency_list::Graph, tree::TreeGraph};
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Grl5A;
impl verify::Solver for Grl5A {
    const PROBLEM_ID: &'static str = "GRL_5_A";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let stw = reader.vec3::<usize, usize, i64>(n - 1);
        let mut graph = Graph::new(n);
        for (s, t, w) in stw {
            graph.add_edge(s, t, w);
        }
        let (_l, _r, d) = graph.diameter();
        writeln!(write, "{}", d).unwrap()
    }
}

#[test]
fn test() {
    Grl5A::assert(
        "4
        0 1 2
        1 2 1
        1 3 3",
        "5",
    );
    Grl5A::assert(
        "4
        0 1 1
        1 2 2
        2 3 4",
        "7",
    );
}
