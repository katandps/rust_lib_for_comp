use rust_lib_for_comp::graph::directed_acyclic_graph::Dag;
use rust_lib_for_comp::graph::strongly_connected_components::SCC;
use rust_lib_for_comp::util::string_util::JoinTrait;
use rust_lib_for_comp::{graph::adjacency_list::Graph, util::io_util::*};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct StronglyConnectedComponents;
impl verify::Solver for StronglyConnectedComponents {
    const PROBLEM_ID: &'static str = "scc";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m) = reader.v2::<usize, usize>();
        let ab = reader.vec2::<usize, usize>(m);
        let mut graph = Graph::new(n);
        for (a, b) in ab {
            graph.add_arc(a, b, 1);
        }
        let scc = SCC::build(&graph);
        scc.graph.topological_sort();
        writeln!(write, "{}", scc.n).ok();
        let mut ans = vec![Vec::new(); scc.n];
        for i in 0..n {
            ans[scc.group[i]].push(i);
        }
        for v in ans {
            writeln!(write, "{} {}", v.len(), v.join(" ")).ok();
        }
    }
}
#[test]
fn test() {
    StronglyConnectedComponents::assert(
        "6 7
        1 4
        5 2
        3 0
        5 5
        4 1
        0 3
        4 2",
        "4
        1 5
        2 1 4
        1 2
        2 0 3",
    );
}
