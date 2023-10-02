// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_C
// verification-helper: ERROR 0.000000001

fn main() {
    solve(io_util::IO::default())
}

use circle::*;
use io_util::*;
use plane_float::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let xy = io.vec2::<f64, f64>(3);
    let p: Vec<_> = xy.into_iter().map(Vector::from).collect();
    let tri = Triangle::new(p[0], p[1], p[2]);
    let circle = tri.circumscribed_circle().unwrap();
    io.out(format!(
        "{} {} {}\n",
        circle.center.x.0, circle.center.y.0, circle.radius.0
    ));

    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "1 -2
        3 2
        -2 0",
        "0.62500000000000000000 0.68750000000000000000 2.71353666826155124291",
    ));
    solve(io_debug::IODebug::fvalue_assert(
        "0 3
    4 0
    0 0",
        "2.00000000000000000000 1.50000000000000000000 2.50000000000000000000",
    ))
}
