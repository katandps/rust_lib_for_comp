// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DSL_5_B
use aoj_solver::dsl_5_b::solve;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let lr = io.vec4::<usize, usize, usize, usize>(n);
    let ans = solve(n, &lr);
    io.out(ans.line());
    io.flush();
}
