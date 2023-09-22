// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_B
use io_util::*;
use plane_float::{Segment, Vector};
use string_util::*;
fn main() {
    let mut io = IO::default();
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = io.v4::<f64, f64, f64, f64>();
        let (s1, s2) = (
            Segment::new(Vector::new(x1, y1), Vector::new(x2, y2)),
            Segment::new(Vector::new(x3, y3), Vector::new(x4, y4)),
        );
        io.out(usize::from(Segment::is_intersect(s1, s2)).line())
    }
    io.flush();
}
