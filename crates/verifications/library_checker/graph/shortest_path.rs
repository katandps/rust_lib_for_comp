use rust_lib_for_comp::{
    graph::{adjacency_list::Graph, dijkstra::Dijkstra},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct ShortestPath;
impl verify::Solver for ShortestPath {
    const PROBLEM_ID: &'static str = "shortest_path";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m, s, t) = reader.v4::<usize, usize, usize, usize>();
        let abc = reader.vec3::<usize, usize, i64>(m);
        let mut graph = Graph::new(n);
        for (a, b, c) in abc {
            graph.add_arc(a, b, c);
        }
        let dijkstra = Dijkstra::calc(&graph, s);
        if dijkstra.dist[t] == i64::MAX {
            writeln!(write, "-1").ok();
        } else {
            let path = dijkstra.path(t);
            writeln!(write, "{} {}", dijkstra.dist[t], path.len() - 1).ok();
            for i in 1..path.len() {
                writeln!(write, "{} {}", path[i - 1], path[i]).ok();
            }
        }
    }
}
#[test]
fn test() {
    ShortestPath::assert(
        "5 7 2 3
        0 3 5
        0 4 3
        2 4 2
        4 3 10
        4 0 7
        2 1 5
        1 0 1",
        "11 3
        2 1
        1 0
        0 3",
    );
    ShortestPath::assert(
        "2 1 0 1
        1 0 10",
        "-1",
    );
}
