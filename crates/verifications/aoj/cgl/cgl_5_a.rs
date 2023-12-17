//! # Closest Pair(最近点対)

use rust_lib_for_comp::geometry::closest_pair::*;
use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_5_A", eps = "1e-6")]
pub fn cgl_5_a(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let n = reader.v::<usize>();
    let xy = reader.vec2::<f64, f64>(n);
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let (dist, _pair) = ClosestPair::closest_pair(points);
    writeln!(write, "{dist}").ok();
    write.flush().ok();
}
