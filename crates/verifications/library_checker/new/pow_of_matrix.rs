use rust_lib_for_comp::algebra::matrix::Matrix;
use rust_lib_for_comp::algebra::mod_int::ModInt;
use rust_lib_for_comp::algebra::Pow;
use rust_lib_for_comp::util::io_util::{ReadHelper, ReaderTrait};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct PowOfMatrix;
impl verify::Solver for PowOfMatrix {
    const PROBLEM_ID: &'static str = "pow_of_matrix";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, k) = reader.v2::<usize, i64>();
        let a = reader.matrix::<ModInt<998_244_353>>(n, n);
        let mat = Matrix::build(a).unwrap();
        let result = mat.pow(k);
        writeln!(write, "{}", result.pointer().shrink(n, n)).unwrap()
    }
}
#[test]
fn test() {
    PowOfMatrix::assert(
        "2 7
        0 1
        1 1",
        "8 13
        13 21",
    );
    PowOfMatrix::assert(
        "3 0
        0 0 0
        0 0 0
        0 0 0",
        "1 0 0
        0 1 0
        0 0 1",
    );
}
