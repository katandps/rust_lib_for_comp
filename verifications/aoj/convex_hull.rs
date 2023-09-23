// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_4_A
use aoj_solver::cgl_4_a::solve;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let ans = solve(n, &xy);
    io.out(ans.len().line());
    for (x, y) in ans {
        io.out(format!("{} {}", x, y).line());
    }
    io.flush();
}
