// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/GRL_6_A
use aoj_solver::grl_6_a::solve;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (v, e) = io.v2::<usize, usize>();
    let uvc = io.vec3::<usize, usize, i64>(e);
    let ans = solve(v, e, &uvc);
    io.out(ans.line());
    io.flush()
}
