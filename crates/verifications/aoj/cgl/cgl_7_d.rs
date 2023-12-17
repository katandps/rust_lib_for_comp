//! # Cross Points of a Circe and a Line(円と直線の交点)

use rust_lib_for_comp::geometry::circle::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::util::string_util::*;

#[verify::aizu_online_judge("CGL_7_D", eps = "1e-8")]
pub fn cgl_7_d(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let (cx, cy, r) = reader.v3::<f64, f64, f64>();
    let c = Circle::new(cx, cy, r);
    for _ in 0..reader.v::<usize>() {
        let (x1, y1, x2, y2) = reader.v4::<f64, f64, f64, f64>();
        let l = Line::new(Vector::new(x1, y1), Vector::new(x2, y2));
        let mut ans = c.cross_point_to_line(&l);
        ans.sort();
        let mut v = Vec::new();
        for i in 0..2 {
            v.push(ans[i % ans.len()].x);
            v.push(ans[i % ans.len()].y);
        }
        writeln!(write, "{}", v.iter().join(" ")).ok();
    }
    write.flush().ok();
}
