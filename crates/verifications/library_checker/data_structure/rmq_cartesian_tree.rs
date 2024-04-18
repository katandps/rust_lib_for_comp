use rust_lib_for_comp::{
    graph::tree::{cartesian_tree::CartesianTree, lowest_common_ancestor::LowestCommonAncestor},
    util::io_util::*,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct StaticRMQCartesianTree;
impl verify::Solver for StaticRMQCartesianTree {
    const PROBLEM_ID: &'static str = "staticrmq";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<i64>(n);
        let ct = CartesianTree::build(&a);
        let mut lca = LowestCommonAncestor::new(&ct.graph, ct.root);
        for _ in 0..q {
            let (l, r) = reader.v2::<usize, usize>();
            writeln!(write, "{}", a[lca.query(l, r - 1)]).ok();
        }
    }
}
#[test]
fn test() {
    StaticRMQCartesianTree::assert(
        "4 10
        2 10 1 100
        0 1
        0 2
        0 3
        0 4
        1 2
        1 3
        1 4
        2 3
        2 4
        3 4",
        "2
        2
        1
        1
        10
        1
        1
        1
        1
        100",
    );
}
