//! <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_D>
use circle::Circle;
use io_util::*;
use plane_float::{Line, Vector};
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
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

#[test]
fn test() {
    solve(io_debug::IODebug::fvalue_assert(
        "2 1 1
        2
        0 1 4 1
        3 0 3 3",
        "1.00000000 1.00000000 3.00000000 1.00000000
        3.00000000 1.00000000 3.00000000 1.00000000",
    ));
}
