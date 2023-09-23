// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_D
// verification-helper: ERROR 0.0000000001
use aoj_solver::cgl_2_d::solve;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = io.v4::<f64, f64, f64, f64>();
        io.out(solve((x1, y1), (x2, y2), (x3, y3), (x4, y4)).line());
    }
    io.flush();
}
