//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_2_A>
use io_util::*;
use plane_float::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v::<usize>() {
        let p0 = io.v2::<f64, f64>();
        let p1 = io.v2::<f64, f64>();
        let p2 = io.v2::<f64, f64>();
        let p3 = io.v2::<f64, f64>();
        let (l1, l2) = (
            Line::new(p0.into(), p1.into()),
            Line::new(p2.into(), p3.into()),
        );
        let ans = if Line::is_parallel(l1, l2) {
            2
        } else if Line::is_orthogonal(l1, l2) {
            1
        } else {
            0
        };
        io.out(ans.line())
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3
        0 0 3 0 0 2 3 2
        0 0 3 0 1 1 1 4
        0 0 3 0 1 1 2 2",
        "2
        1
        0",
    ))
}
