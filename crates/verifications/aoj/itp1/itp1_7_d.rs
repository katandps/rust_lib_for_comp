//! # Matrix Multiplication(行列の積)

use rust_lib_for_comp::algebra::matrix::Matrix;
use rust_lib_for_comp::util::io_util::*;

//#[verify::aizu_online_judge("ITP1_7_D")]
pub fn itp1_7_d(read: impl std::io::Read, mut write: impl std::io::Write) {
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
