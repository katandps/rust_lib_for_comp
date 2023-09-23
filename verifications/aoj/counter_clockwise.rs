// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_1_C
use aoj_solver::cgl_1_c::solve;
use io_util::*;
use plane_float::ClockwiseDirection;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
    let q = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(q);
    for a in solve((x1, y1), (x2, y2), q, &xy) {
        let res = match a {
            ClockwiseDirection::Clockwise => "CLOCKWISE",
            ClockwiseDirection::CounterClockwise => "COUNTER_CLOCKWISE",
            ClockwiseDirection::OneLineCAB => "ONLINE_BACK",
            ClockwiseDirection::OneLineABC => "ONLINE_FRONT",
            ClockwiseDirection::OneLineACB => "ON_SEGMENT",
        };
        io.out(res.line())
    }
    io.flush();
}
