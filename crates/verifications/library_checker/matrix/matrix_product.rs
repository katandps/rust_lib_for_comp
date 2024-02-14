use rust_lib_for_comp::{
    algebra::{matrix::Matrix, mod_int::ModInt},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct MatrixProduct;
impl verify::Solver for MatrixProduct {
    const PROBLEM_ID: &'static str = "matrix_product";
    const TIME_LIMIT_MILLIS: u64 = 10000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m, k) = reader.v3::<usize, usize, usize>();
        let a = reader.matrix::<ModInt>(n, m);
        let b = reader.matrix::<ModInt>(m, k);
        let am = Matrix::build(a).unwrap();
        let bm = Matrix::build(b).unwrap();
        let c = (am * bm).unwrap();
        writeln!(write, "{c}").ok();
    }
}
#[test]
fn test() {
    MatrixProduct::assert(
        "2 2 2
        1 1
        1 0
        5 2
        3 1",
        "8 3
        5 2",
    );
    MatrixProduct::assert(
        "1 2 3
        1 2
        3 4 5
        6 7 8",
        "15 18 21",
    );
    MatrixProduct::assert(
        "1 1 1
        123456
        789012",
        "578563231",
    );
    MatrixProduct::assert(
        "4 4 4
    1 2 3 4
    5 6 7 8
    9 10 11 12
    13 14 15 16
    1 2 3 4
    5 6 7 8
    9 10 11 12
    13 14 15 16",
        "90 100 110 120
        202 228 254 280
        314 356 398 440
        426 484 542 600",
    )
}
