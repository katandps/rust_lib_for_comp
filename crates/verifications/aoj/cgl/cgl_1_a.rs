//! # Projection(射影)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_1_A", eps = "1e-8")]
pub fn cgl_1_a(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let (x1, y1, x2, y2) = reader.v4::<f64, f64, f64, f64>();
    let line = Line::new(Vector::new(x1, y1), Vector::new(x2, y2));
    for _ in 0..reader.v::<usize>() {
        let (x, y) = reader.v2::<f64, f64>();
        let p = Vector::new(x, y);
        let result = line.projection(p);
        writeln!(write, "{} {}", result.x, result.y).ok();
    }
    write.flush().ok();
}
