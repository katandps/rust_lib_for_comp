use rust_lib_for_comp::{
    graph::{adjacency_list::Graph, tree::lowest_common_ancestor::LowestCommonAncestor},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct LowestCommonAncestorSolver;
impl verify::Solver for LowestCommonAncestorSolver {
    const PROBLEM_ID: &'static str = "lca";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let mut graph = Graph::new(n);
        for i in 1..n {
            graph.add_edge(i, reader.v(), ());
        }
        let mut lca = LowestCommonAncestor::new(&graph, 0);
        for _ in 0..q {
            let (u, v) = reader.v2::<usize, usize>();
            writeln!(write, "{}", lca.query(u, v)).ok();
        }
    }
}
#[test]
fn test() {
    LowestCommonAncestorSolver::assert(
        "5 5
        0 0 2 2
        0 1
        0 4
        1 2
        2 3
        3 4",
        "0
        0
        0
        2
        2",
    );
}
