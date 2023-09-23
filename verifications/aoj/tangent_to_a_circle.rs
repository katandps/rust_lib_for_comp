// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_F
// verification-helper: ERROR 0.00000001

use aoj_solver::cgl_7_f::solve;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let p = io.v2::<f64, f64>();
    let c = io.v2::<f64, f64>();
    let r = io.v::<f64>();
    let ans = solve(p, c, r);

    io.out(format!("{} {}", ans.0 .0, ans.0 .1).line());
    io.out(format!("{} {}", ans.1 .0, ans.1 .1).line());
    io.flush();
}
