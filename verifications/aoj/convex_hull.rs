// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_4_A
use convex_hull::cgl_4_a;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    let xy = io.vec2::<f64, f64>(n);
    let ans = cgl_4_a(n, &xy);
    io.out(ans.len().line());
    for (x, y) in ans {
        io.out(format!("{} {}", x, y).line());
    }
    io.flush();
}
