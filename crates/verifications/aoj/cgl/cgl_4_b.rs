//! # Diameter of a Convex Polygon(凸多角形の直径)

use rust_lib_for_comp::geometry::convex_hull::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_4_B", eps = "1e-6")]
pub fn cgl_4_b(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let n = reader.v::<usize>();
    let xy = reader.vec2::<f64, f64>(n);
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let polygon = Polygon::convex_hull(points, true);
    writeln!(write, "{}", polygon.diameter()).ok();
    write.flush().ok();
}
