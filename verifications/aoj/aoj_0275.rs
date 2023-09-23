// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/0275

use aoj_solver::aoj_0275::solve;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (s, r) = io.v2::<usize, usize>();
    let uvw = io.vec3::<usize, usize, i64>(r);
    let (a, b, q) = io.v3::<usize, usize, usize>();
    let cd = io.vec2::<usize, usize>(q);
    let bools = solve(s, &uvw, a, b, &cd);
    for b in bools {
        io.out(if b { "Yes" } else { "No" }.line());
    }
    io.flush()
}
