//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_C>
use convex_hull::Polygon;
use io_util::*;
use plane_float::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let mut p = Vec::new();
    let q = io.v::<usize>();
    for _ in 0..q {
        p.push(io.v4::<f64, f64, f64, f64>());
    }
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let polygon = Polygon::convex_hull(points, true);
    let mut ret = Vec::new();
    for (p1x, p1y, p2x, p2y) in p {
        let line = Line::new(Vector::new(p1x, p1y), Vector::new(p2x, p2y));
        let ans = polygon.cut(line);
        assert!(ans.is_convex());
        ret.push(ans.area());
    }
    for ans in ret {
        io.out(ans.line())
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "4
        1 1
        4 1
        4 3
        1 3
        2
        2 0 2 4
        2 4 2 0",
        "2.00000000
        4.00000000",
    ));
}
