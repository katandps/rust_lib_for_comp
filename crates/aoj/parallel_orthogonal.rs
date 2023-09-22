// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_2_A
use io_util::*;
use plane_float::cgl_2_a;
use string_util::*;

fn main() {
    let mut io = IO::default();
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = io.v4::<f64, f64, f64, f64>();
        let result = cgl_2_a((x1, y1), (x2, y2), (x3, y3), (x4, y4));
        io.out(result.line())
    }
    io.flush();
}
