//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_3_A>
use convex_hull::Polygon;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    io.out(polygon.area().line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3
        0 0
        2 2
        -1 1",
        "2",
    ))
}
