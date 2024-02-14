use rust_lib_for_comp::{
    graph::{adjacency_list::Graph, dijkstra::Dijkstra},
    util::{
        io_util::{ReadHelper, ReaderTrait},
        string_util::JoinTrait,
    },
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct TreeDiameter;
impl verify::Solver for TreeDiameter {
    const PROBLEM_ID: &'static str = "tree_diameter";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let abc = reader.vec3::<usize, usize, i64>(n - 1);
        let mut graph = Graph::new(n);
        for (a, b, c) in abc {
            graph.add_edge(a, b, c);
        }
        let (dist, _l, r) = Dijkstra::diameter(&graph);
        let path = dist.path(r);
        writeln!(write, "{} {}", dist.dist[r], path.len()).ok();
        writeln!(write, "{}", path.join(" ")).ok();
    }
}
#[test]
fn test() {
    TreeDiameter::assert(
        "8
        0 1 5
        1 2 3
        2 3 1
        1 4 2
        4 7 4
        1 5 7
        2 6 5",
        "15 4
        6 2 1 5",
    );
}
