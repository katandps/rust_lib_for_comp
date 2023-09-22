// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_D
// verification-helper: ERROR 0.000001

use circle::Circle;
use io_util::*;
use plane_float::{Line, Vector};
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (cx, cy, r) = io.v3::<f64, f64, f64>();
    let c = Circle::new(cx, cy, r);
    for _ in 0..io.v::<usize>() {
        let (x1, y1, x2, y2) = io.v4::<f64, f64, f64, f64>();
        let l = Line::new(Vector::new(x1, y1), Vector::new(x2, y2));
        let mut ans = c.cross_point_to_line(&l);
        ans.sort();
        let mut v = Vec::new();
        for i in 0..2 {
            v.push(ans[i % ans.len()].x);
            v.push(ans[i % ans.len()].y);
        }
        io.out(v.join(" ").line());
    }
    io.flush();
}
