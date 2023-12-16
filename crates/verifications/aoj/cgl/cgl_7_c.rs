//! # Circumscribed Circle of a Triangle(外接円)

use rust_lib_for_comp::geometry::{circle::*, plane_float::*};
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_7_C", eps = "1e-6")]
pub fn cgl_7_c(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::default().add(read);
    let xy = reader.vec2::<f64, f64>(3);
    let p: Vec<_> = xy.into_iter().map(Vector::from).collect();
    let tri = Triangle::new(p[0], p[1], p[2]);
    let circle = tri.circumscribed_circle().unwrap();
    writeln!(
        write,
        "{} {} {}",
        circle.center.x.0, circle.center.y.0, circle.radius.0
    )
    .ok();
    write.flush().ok();
}
