//! # Tangent to a Circle(円の接線)

use rust_lib_for_comp::geometry::circle::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Cgl7F;
impl verify::Solver for Cgl7F {
    const PROBLEM_ID: &'static str = "CGL_7_F";
    const EPSILON: Option<f64> = Some(1e-5);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
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
}
