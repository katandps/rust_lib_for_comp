//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_C>
use io_util::*;
use plane_float::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = io.v4::<f64, f64, f64, f64>();
        if let Some(result) = Segment::cross_point(
            Segment::new((x1, y1).into(), (x2, y2).into()),
            Segment::new((x3, y3).into(), (x4, y4).into()),
        ) {
            io.out(format!("{} {}\n", result.x, result.y));
        } else {
            panic!("line is parallel")
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "3
        0 0 2 0 1 1 1 -1
        0 0 1 1 0 1 1 0
        0 0 1 1 1 0 0 1",
        "1.0000000000 0.0000000000
        0.5000000000 0.5000000000
        0.5000000000 0.5000000000",
    ))
}
#[test]
#[should_panic]
fn unreachable() {
    solve(io_debug::IODebug::static_assert(
        "1
        0 0 1 1 0 1 1 2",
        "unreachable",
    ))
}
