// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/3/CGL_3_C
use convex_hull::{Including, Polygon};
use io_util::*;
use plane_float::Point;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    for _ in 0..io.v::<usize>() {
        let p = Point::from(io.v2::<f64, f64>());
        io.out(
            match polygon.include(p) {
                Including::Inside => 2,
                Including::OnLine => 1,
                Including::Outside => 0,
            }
            .line(),
        )
    }
    io.flush();
}
