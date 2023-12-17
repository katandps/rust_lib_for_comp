//! # Convex Cut(凸多角形の切断)

use rust_lib_for_comp::geometry::convex_hull::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_4_C", eps = "1e-8")]
pub fn cgl_4_c(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let n = reader.v::<usize>();
    let xy = reader.vec2::<f64, f64>(n);
    let mut p = Vec::new();
    let q = reader.v::<usize>();
    for _ in 0..q {
        p.push(reader.v4::<f64, f64, f64, f64>());
    }
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let polygon = Polygon::convex_hull(points, true);
    let mut ret = Vec::new();
    for (p1x, p1y, p2x, p2y) in p {
        let line = Line::new(Vector::new(p1x, p1y), Vector::new(p2x, p2y));
        let ans = polygon.cut(line);
        assert!(ans.is_convex());
        ret.push(ans.area());
    }
    for ans in ret {
        writeln!(write, "{ans}").ok();
    }
    write.flush().ok();
}
