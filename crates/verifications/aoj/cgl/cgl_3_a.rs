//! # Area
use rust_lib_for_comp::geometry::convex_hull::Polygon;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_3_A")]
pub fn grl_1_c(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::default().add(read);
    let xy = reader.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    writeln!(write, "{}", polygon.area()).ok();
    write.flush().ok();
}
