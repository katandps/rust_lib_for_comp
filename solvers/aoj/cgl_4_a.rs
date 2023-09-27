//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_4_A>
use convex_hull::*;
use io_util::*;
use plane_float::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let points = xy
        .iter()
        .cloned()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let convex_hull = Polygon::convex_hull(points, true);
    let mut poly = Polygon::new(convex_hull.nodes.into_iter().map(Vector::swap).collect());
    // yについて正規化
    poly.normalize();
    let ans = Polygon::new(poly.nodes.into_iter().map(Vector::swap).collect());
    io.out(ans.nodes.len().line());
    for v in ans.nodes {
        io.out(format!("{} {}", v.x, v.y).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "7
        2 1
        0 0
        1 2
        2 2
        4 2
        1 3
        3 3",
        "5
        0 0
        2 1
        4 2
        3 3
        1 3",
    ));
}
