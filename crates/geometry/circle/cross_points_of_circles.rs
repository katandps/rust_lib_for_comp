// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_E
// verification-helper: ERROR 0.000001

use circle::Circle;
use io_util::*;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (cx, cy, r) = io.v3::<f64, f64, f64>();
    let c1 = Circle::new(cx, cy, r);
    let (cx, cy, r) = io.v3::<f64, f64, f64>();
    let c2 = Circle::new(cx, cy, r);
    let mut ans = c1.cross_point_to_circle(&c2);
    ans.sort();
    let mut v = Vec::new();
    for i in 0..2 {
        v.push(ans[i % ans.len()].x);
        v.push(ans[i % ans.len()].y);
    }
    io.out(v.join(" ").line());

    io.flush();
}
