//! # Intersection(交差判定)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_2_B")]
pub fn cgl_2_b(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    for _ in 0..reader.v::<usize>() {
        let (x1, y1, x2, y2) = reader.v4::<f64, f64, f64, f64>();
        let (x3, y3, x4, y4) = reader.v4::<f64, f64, f64, f64>();
        let (s1, s2) = (
            Segment::new(Vector::new(x1, y1), Vector::new(x2, y2)),
            Segment::new(Vector::new(x3, y3), Vector::new(x4, y4)),
        );
        writeln!(write, "{}", usize::from(Segment::is_intersect(s1, s2))).ok();
    }
    write.flush().ok();
}
