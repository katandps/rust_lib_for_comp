//! # Distance(距離)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_2_D", eps = "1e-8")]
pub fn cgl_2_d(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    for _ in 0..reader.v::<usize>() {
        let p0 = reader.v2::<f64, f64>();
        let p1 = reader.v2::<f64, f64>();
        let p2 = reader.v2::<f64, f64>();
        let p3 = reader.v2::<f64, f64>();
        let ans = Segment::distance(
            Segment::new(p0.into(), p1.into()),
            Segment::new(p2.into(), p3.into()),
        );
        writeln!(write, "{ans}").ok();
    }
    write.flush().ok();
}
