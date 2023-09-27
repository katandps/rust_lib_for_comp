//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_1_C>
use io_util::*;
use plane_float::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (p1, p2) = (io.v2::<f64, f64>(), io.v2::<f64, f64>());
    let q = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(q);
    let (p1, p2) = (p1.into(), p2.into());
    for xy in xy {
        let p3 = xy.into();
        let ans = match ClockwiseDirection::direction(p1, p2, p3) {
            ClockwiseDirection::Clockwise => "CLOCKWISE",
            ClockwiseDirection::CounterClockwise => "COUNTER_CLOCKWISE",
            ClockwiseDirection::OneLineCAB => "ONLINE_BACK",
            ClockwiseDirection::OneLineABC => "ONLINE_FRONT",
            ClockwiseDirection::OneLineACB => "ON_SEGMENT",
        };
        io.out(ans.line())
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "0 0 2 0
        5
        -1 1
        -1 -1
        -1 0
        0 0
        3 0",
        "COUNTER_CLOCKWISE
        CLOCKWISE
        ONLINE_BACK
        ON_SEGMENT
        ONLINE_FRONT",
    ))
}
