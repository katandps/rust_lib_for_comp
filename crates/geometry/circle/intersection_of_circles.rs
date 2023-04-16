// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_A
use circle::{Circle, CircleIntersection};
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (x1, y1, r1) = io.v3::<f64, f64, f64>();
    let (x2, y2, r2) = io.v3::<f64, f64, f64>();
    let c1 = Circle::new(x1, y1, r1);
    let c2 = Circle::new(x2, y2, r2);
    io.out(
        match CircleIntersection::intersect(&c1, &c2) {
            CircleIntersection::NotCross => 4,
            CircleIntersection::Circumscribed => 3,
            CircleIntersection::Intersect => 2,
            CircleIntersection::Inscribed => 1,
            CircleIntersection::Included => 0,
        }
        .line(),
    );
    io.flush();
}
