//! # Tangent to a Circle(円の接線)

use rust_lib_for_comp::geometry::circle::*;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("CGL_7_F", eps = "1e-5")]
pub fn cgl_7_f(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let p = reader.v2::<f64, f64>();
    let c = reader.v2::<f64, f64>();
    let r = reader.v::<f64>();
    let p = p.into();
    let c = Circle::new(c.0, c.1, r);

    let mut ans = c.tangent(p);
    ans.sort();
    writeln!(write, "{} {}", ans[0].x, ans[0].y).ok();
    writeln!(write, "{} {}", ans[1].x, ans[1].y).ok();
    write.flush().ok();
}
