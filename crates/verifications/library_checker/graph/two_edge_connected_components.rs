use rust_lib_for_comp::{
    graph::{adjacency_list::Graph, two_edge_connected_components::TwoEdgeConnectedComponents},
    util::{
        io_util::{ReadHelper, ReaderTrait},
        string_util::JoinTrait,
    },
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct TwoEdgeConnectedComponentsVerify;
impl verify::Solver for TwoEdgeConnectedComponentsVerify {
    const PROBLEM_ID: &'static str = "two_edge_connected_components";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m) = reader.v2::<usize, usize>();
        let ab = reader.vec2::<usize, usize>(m);
        let mut graph = Graph::new(n);
        for (a, b) in ab {
            graph.add_edge(a, b, ());
        }
        let tecc = TwoEdgeConnectedComponents::build(&graph);
        let mut ans = Vec::new();
        for i in 0..n {
            let k = tecc.group(i);
            while ans.len() <= k {
                ans.push(Vec::new())
            }
            ans[k].push(i);
        }
        writeln!(write, "{}", ans.len()).unwrap();
        for v in ans {
            writeln!(write, "{} {}", v.len(), v.join(" ")).unwrap()
        }
    }
}
#[test]
fn test() {
    TwoEdgeConnectedComponentsVerify::assert(
        "4 5
        0 2
        0 1
        3 0
        2 1
        2 3",
        "1
        4 0 1 2 3",
    );
    TwoEdgeConnectedComponentsVerify::assert(
        "13 21
        4 5
        8 7
        12 3
        3 10
        1 5
        10 2
        0 0
        11 4
        2 12
        9 1
        9 0
        7 8
        7 6
        9 1
        8 2
        12 10
        11 0
        8 6
        3 2
        5 9
        4 11",
        "3
        6 0 1 4 5 9 11
        4 2 3 10 12
        3 6 7 8",
    );
    TwoEdgeConnectedComponentsVerify::assert(
        "2 2
    0 1
    1 0",
        "1
        2 0 1",
    )
}
