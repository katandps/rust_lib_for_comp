// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_C
// verification-helper: ERROR 0.00000001
use convex_hull::cgl_4_c;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let mut p = Vec::new();
    let q = io.v::<usize>();
    for _ in 0..q {
        p.push(io.v4::<f64, f64, f64, f64>());
    }
    for ans in cgl_4_c(n, xy, q, p) {
        io.out(ans.line())
    }
    io.flush();
}
