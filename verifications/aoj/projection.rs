// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_1_A
// verification-helper: ERROR 0.0000000001
use io_util::*;
use plane_float::{Line, Vector};

fn main() {
    let mut io = IO::default();
    let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
    let line = Line::new(Vector::new(x1, y1), Vector::new(x2, y2));
    for _ in 0..io.v::<usize>() {
        let (x, y) = io.v2::<f64, f64>();
        let p = Vector::new(x, y);
        let result = line.projection(p);
        io.out(format!("{} {}\n", result.x, result.y))
    }
    io.flush();
}
