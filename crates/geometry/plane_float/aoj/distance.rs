// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_D
// verification-helper: ERROR 0.0000000001
use io_util::*;
use plane_float::{Point, Segment};
use string_util::*;

fn main() {
    let mut io = IO::default();
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = io.v4::<f64, f64, f64, f64>();
        let result = Segment::distance(
            Segment::new(Point::new(x1, y1), Point::new(x2, y2)),
            Segment::new(Point::new(x3, y3), Point::new(x4, y4)),
        );
        io.out(result.line());
    }
    io.flush();
}
