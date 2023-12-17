//! # Parallel/Orthogonal(平行・垂直)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_2_A")]
pub fn cgl_2_a(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    for _ in 0..reader.v::<usize>() {
        let p0 = reader.v2::<f64, f64>();
        let p1 = reader.v2::<f64, f64>();
        let p2 = reader.v2::<f64, f64>();
        let p3 = reader.v2::<f64, f64>();
        let (l1, l2) = (
            Line::new(p0.into(), p1.into()),
            Line::new(p2.into(), p3.into()),
        );
        let ans = if Line::is_parallel(l1, l2) {
            2
        } else if Line::is_orthogonal(l1, l2) {
            1
        } else {
            0
        };
        writeln!(write, "{ans}").ok();
    }
    write.flush().ok();
}
