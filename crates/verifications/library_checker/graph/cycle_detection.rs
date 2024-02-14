use rust_lib_for_comp::{
    graph::{adjacency_list::Graph, find_cycle::FindCycle},
    util::{io_util::*, string_util::JoinTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct CycleDetection;
impl verify::Solver for CycleDetection {
    const PROBLEM_ID: &'static str = "cycle_detection";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m) = reader.v2::<usize, usize>();
        let uv = reader.vec2::<usize, usize>(m);
        let mut graph = Graph::new(n);
        for (u, v) in uv {
            graph.add_arc(u, v, ());
        }
        let cycle = graph.find_cycle(false);
        if let Some(v) = cycle {
            writeln!(write, "{}", v.len()).ok();
            writeln!(write, "{}", v.join("\n")).ok();
        } else {
            writeln!(write, "-1").ok();
        }
    }
}
#[test]
fn test() {
    CycleDetection::assert(
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
    );
    CycleDetection::assert(
        "2 1
        1 0",
        "-1",
    );
    CycleDetection::assert(
        "4 6
        0 1
        1 2
        2 0
        0 1
        1 3
        3 0",
        "3
        2 0 1",
    )
}
