//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_B>
use circle::*;
use io_util::*;
use plane_float::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let xy = io.vec2::<f64, f64>(3);
    let p: Vec<_> = xy.into_iter().map(Vector::from).collect();
    let tri = Triangle::new(p[0], p[1], p[2]);
    let circle = tri.inner_circle().unwrap();
    io.out(format!(
        "{} {} {}\n",
        circle.center.x, circle.center.y, circle.radius
    ));
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "1 -2
        3 2
        -2 0",
        "0.53907943898209422325 -0.26437392711448356856 1.18845545916395465278",
    ));
    solve(io_debug::IODebug::fvalue_assert(
        "0 3
        4 0
        0 0",
        "1.00000000000000000000 1.00000000000000000000 1.00000000000000000000",
    ))
}
