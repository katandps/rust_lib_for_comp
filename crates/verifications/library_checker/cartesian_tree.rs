use rust_lib_for_comp::{
    graph::{tree::cartesian_tree::CartesianTree, GraphTrait},
    util::{io_util::*, string_util::JoinTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct CartesianTreeSolver;
impl verify::Solver for CartesianTreeSolver {
    const PROBLEM_ID: &'static str = "cartesian_tree";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let a = reader.vec::<usize>(n);
        let ct = CartesianTree::build(&a);
        let mut ans = vec![0; n];
        ans[ct.root] = ct.root;
        for src in 0..n {
            for (dst, _) in ct.graph.edges(src) {
                ans[dst] = src;
            }
        }
        writeln!(write, "{}", ans.join(" ")).ok();
    }
}
#[test]
fn test() {
    CartesianTreeSolver::assert(
        "3
        1 0 2",
        "1 1 1",
    );
    CartesianTreeSolver::assert(
        "11
        9 3 7 1 8 12 10 20 15 18 5",
        "1 3 1 3 10 6 4 8 6 8 3",
    )
}
