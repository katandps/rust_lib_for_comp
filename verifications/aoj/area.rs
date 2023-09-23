// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_3_A
// verification-helper: ERROR 0.1
use aoj_solver::cgl_3_a::solve;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    io.out(solve(n, xy).line());
    io.flush();
}
