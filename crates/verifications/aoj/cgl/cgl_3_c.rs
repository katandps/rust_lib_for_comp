//! # Polygon-Point-Containment(多角形-点の包含)

use rust_lib_for_comp::geometry::convex_hull::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_3_C")]
pub fn cgl_3_c(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let n = reader.v::<usize>();
    let xy = reader.vec2::<f64, f64>(n);
    let polygon = Polygon::from(&xy[..]);
    for _ in 0..reader.v::<usize>() {
        let p = Vector::from(reader.v2::<f64, f64>());
        writeln!(
            write,
            "{}",
            match polygon.include(p) {
                Including::Inside => 2,
                Including::OnLine => 1,
                Including::Outside => 0,
            },
        )
        .ok();
    }
    write.flush().ok();
}
