// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_C
// verification-helper: ERROR 0.000000001

use circle::Triangle;
use io_util::*;
use plane_float::Vector;

fn main() {
    let mut io = IO::default();
    let xy = io.vec2::<f64, f64>(3);
    let p: Vec<_> = xy.into_iter().map(Vector::from).collect();
    let tri = Triangle::new(p[0], p[1], p[2]);
    let circle = tri.circumscribed_circle().unwrap();
    io.out(format!(
        "{} {} {}\n",
        circle.center.x, circle.center.y, circle.radius
    ));
    io.flush();
}
