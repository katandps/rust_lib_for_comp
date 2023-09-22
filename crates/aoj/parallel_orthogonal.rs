// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_2_A
use io_util::*;
use plane_float::{Line, Vector};
use string_util::*;

fn main() {
    let mut io = IO::default();
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = io.v4::<f64, f64, f64, f64>();
        let (l1, l2) = (
            Line::new(Vector::new(x1, y1), Vector::new(x2, y2)),
            Line::new(Vector::new(x3, y3), Vector::new(x4, y4)),
        );
        let result = if Line::is_parallel(l1, l2) {
            2
        } else if Line::is_orthogonal(l1, l2) {
            1
        } else {
            0
        };
        io.out(result.line())
    }
    io.flush();
}
