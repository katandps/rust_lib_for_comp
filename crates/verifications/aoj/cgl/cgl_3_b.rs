//! # Is-Convex(凸性判定)

use rust_lib_for_comp::geometry::convex_hull::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_3_B")]
pub fn cgl_3_b(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let n = reader.v::<usize>();
    let xy = reader.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    writeln!(write, "{}", usize::from(polygon.is_convex())).ok();
    write.flush().ok();
}
