// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_F
// verification-helper: ERROR 0.00000001

use circle::Circle;
use io_util::*;
use plane_float::Vector;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (px, py) = io.v2::<f64, f64>();
    let p = Vector::new(px, py);
    let (cx, cy, r) = io.v3::<f64, f64, f64>();
    let c = Circle::new(cx, cy, r);
    let mut ans = c.tangent(p);
    ans.sort();
    for i in 0..2 {
        io.out(format!("{} {}", ans[i % ans.len()].x, ans[i % ans.len()].y).line())
    }
    io.flush();
}
