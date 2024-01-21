use rust_lib_for_comp::{
    algebra::{matrix::Matrix, mod_int::ModInt},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct UnionFindSolver;
impl verify::Solver for UnionFindSolver {
    const PROBLEM_ID: &'static str = "matrix_det";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let a = reader.matrix::<ModInt>(n, n);
        let matrix = Matrix::build(a).unwrap();
        writeln!(write, "{}", matrix.determinant().unwrap()).ok();
    }
}
#[test]
fn test() {
    UnionFindSolver::assert(
        "3
        3 1 4
        1 5 9
        2 6 5",
        "998244263",
    );
    UnionFindSolver::assert(
        "3
        1 2 3
        4 5 6
        7 8 9",
        "0",
    );
    UnionFindSolver::assert(
        "2
        0 1
        1 0",
        "998244352",
    );
}
