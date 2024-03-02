//! # Bridges(橋)

use rust_lib_for_comp::graph::{adjacency_list::Graph, low_link::LowLink};
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::{max, min};
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Grl3B;
impl verify::Solver for Grl3B {
    const PROBLEM_ID: &'static str = "GRL_3_B";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (v, e) = reader.v2::<usize, usize>();
        let st = reader.vec2::<usize, usize>(e);
        let mut graph = Graph::new(v);
        for (s, t) in st {
            graph.add_edge(s, t, ());
        }
        let ll = LowLink::build(&graph);
        for (s, t) in &ll.bridge {
            writeln!(write, "{} {}", min!(s, t), max!(s, t)).unwrap()
        }
    }
}

#[test]
fn test() {
    Grl3B::assert(
        "4 4
0 1
0 2
1 2
2 3",
        "2 3",
    );
    Grl3B::assert(
        "5 4
0 1
1 2
2 3
3 4",
        "0 1
1 2
2 3
3 4",
    );
}
