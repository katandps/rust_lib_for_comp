//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_D>
use io_util::*;
use plane_float::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v::<usize>() {
        let p0 = io.v2::<f64, f64>();
        let p1 = io.v2::<f64, f64>();
        let p2 = io.v2::<f64, f64>();
        let p3 = io.v2::<f64, f64>();
        let ans = Segment::distance(
            Segment::new(p0.into(), p1.into()),
            Segment::new(p2.into(), p3.into()),
        );
        io.out(ans.line())
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "3
        0 0 1 0 0 1 1 1
        0 0 1 0 2 1 1 2
        -1 0 1 0 0 1 0 -1",
        "1.0000000000
        1.4142135624
        0.0000000000",
    ))
}
