// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/5/CGL_5_A
// verification-helper: ERROR 0.00000001
use closest_pair::ClosestPair;
use io_util::*;
use plane_float::Point;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let points = xy
        .into_iter()
        .map(|(x, y)| Point::new(x, y))
        .collect::<Vec<_>>();
    let (dist, _pair) = ClosestPair::closest_pair(points);
    io.out(dist.line());
    io.flush();
}
