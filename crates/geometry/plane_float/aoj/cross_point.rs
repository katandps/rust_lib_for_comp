// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_C
// verification-helper: ERROR 0.0000000001
use io_util::*;
use plane_float::{Segment, Vector};

fn main() {
    let mut io = IO::default();
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = io.v4::<f64, f64, f64, f64>();
        if let Some(result) = Segment::cross_point(
            Segment::new(Vector::new(x1, y1), Vector::new(x2, y2)),
            Segment::new(Vector::new(x3, y3), Vector::new(x4, y4)),
        ) {
            io.out(format!("{} {}\n", result.x, result.y))
        } else {
            unreachable!()
        }
    }
    io.flush();
}
