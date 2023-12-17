//! # Incircle of a Triangle(内接円)

use rust_lib_for_comp::geometry::circle::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_7_B", eps = "1e-6")]
pub fn cgl_7_b(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let xy = reader.vec2::<f64, f64>(3);
    let p: Vec<_> = xy.into_iter().map(Vector::from).collect();
    let tri = Triangle::new(p[0], p[1], p[2]);
    let circle = tri.inner_circle().unwrap();
    writeln!(
        write,
        "{} {} {}",
        circle.center.x, circle.center.y, circle.radius
    )
    .ok();
    write.flush().ok();
}
