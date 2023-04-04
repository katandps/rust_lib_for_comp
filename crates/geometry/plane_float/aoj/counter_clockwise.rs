// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_1_C
use io_util::*;
use plane_float::{ClockwiseDirection, Point};
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
    let (p1, p2) = (Point::new(x1, y1), Point::new(x2, y2));
    for _ in 0..io.v::<usize>() {
        let (x, y) = io.v2::<f64, f64>();
        let p3 = Point::new(x, y);
        let res = match ClockwiseDirection::direction(p1, p2, p3) {
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
