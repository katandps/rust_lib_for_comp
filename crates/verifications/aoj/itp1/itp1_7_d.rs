//! # Matrix Multiplication(行列の積)

use rust_lib_for_comp::algebra::matrix::Matrix;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Itp1_7D;
impl verify::Solver for Itp1_7D {
    const PROBLEM_ID: &'static str = "ITP1_7_D";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m, l) = reader.v3::<usize, usize, usize>();
        let a = reader.matrix::<i64>(n, m);
        let b = reader.matrix::<i64>(m, l);
        let a = Matrix::build(a).unwrap();
        let b = Matrix::build(b).unwrap();
        let c = (a * b).unwrap();
        writeln!(write, "{c}").ok();
        write.flush().ok();
    }
}

#[test]
fn test() {
    Itp1_7D::assert(
        "3 2 3
    1 2
    0 3
    4 5
    1 2 1
    0 3 2",
        "1 8 5
        0 9 6
        4 23 14",
    )
}
