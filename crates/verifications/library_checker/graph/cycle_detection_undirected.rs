use rust_lib_for_comp::{
    graph::{adjacency_list::Graph, find_cycle::FindCycle},
    util::{io_util::*, string_util::JoinTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct CycleDetectionUndirected;
impl verify::Solver for CycleDetectionUndirected {
    const PROBLEM_ID: &'static str = "cycle_detection_undirected";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m) = reader.v2::<usize, usize>();
        let uv = reader.vec2::<usize, usize>(m);
        let mut graph = Graph::new(n);
        for &(u, v) in &uv {
            graph.add_edge(u, v, ());
        }
        let cycle = graph.find_cycle(true);
        if let Some(v) = cycle {
            writeln!(write, "{}", v.len()).ok();
            let mut edges = Vec::new();
            let mut vertices = Vec::new();
            for i in v {
                edges.push(i / 2);
                vertices.push(graph.edges[i].0);
            }
            writeln!(write, "{}", vertices.join(" ")).ok();
            writeln!(write, "{}", edges.join(" ")).ok();
        } else {
            writeln!(write, "-1").ok();
        }
    }
}
#[test]
fn test() {
    CycleDetectionUndirected::assert(
        "6 6
        0 2
        0 3
        4 2
        3 1
        2 1
        2 5",
        "4
        3 0 2 1
        1 0 4 3",
    );
    CycleDetectionUndirected::assert(
        "10 1
        3 3",
        "1
        3
        0",
    );
    CycleDetectionUndirected::assert(
        "10 3
        3 5
        3 5
        5 3",
        "2
        5 3
        1 0",
    );
    CycleDetectionUndirected::assert(
        "6 5
        0 3
        2 0
        1 3
        3 5
        4 2",
        "-1",
    );
    CycleDetectionUndirected::assert("6 0", "-1")
}
