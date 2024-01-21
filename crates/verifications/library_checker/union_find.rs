use rust_lib_for_comp::{
    data_structure::union_find::UnionFind,
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct UnionFindSolver;
impl verify::Solver for UnionFindSolver {
    const PROBLEM_ID: &'static str = "unionfind";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v();
        let mut uf = UnionFind::new(n);
        for _ in 0..reader.v() {
            let (t, u, v) = reader.v3::<usize, usize, usize>();
            if t == 0 {
                uf.unite(u, v);
            } else {
                writeln!(write, "{}", usize::from(uf.same(u, v))).ok();
            }
        }
    }
}
#[test]
fn test() {
    UnionFindSolver::assert(
        "4 7
        1 0 1
        0 0 1
        0 2 3
        1 0 1
        1 1 2
        0 0 2
        1 1 3",
        "0
        1
        0
        1",
    );
}
