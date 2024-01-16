//! # Counter-Clockwise(反時計回り)

use rust_lib_for_comp::geometry::plane_float::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};
#[derive(AizuOnlineJudge)]
pub struct Cgl1C;
impl verify::Solver for Cgl1C {
    const PROBLEM_ID: &'static str = "CGL_1_C";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (p1, p2) = (reader.v2::<f64, f64>(), reader.v2::<f64, f64>());
        let q = reader.v::<usize>();
        let xy = reader.vec2::<f64, f64>(q);
        let (p1, p2) = (p1.into(), p2.into());
        for xy in xy {
            let p3 = xy.into();
            let ans = match ClockwiseDirection::direction(p1, p2, p3) {
                ClockwiseDirection::Clockwise => "CLOCKWISE",
                ClockwiseDirection::CounterClockwise => "COUNTER_CLOCKWISE",
                ClockwiseDirection::OneLineCAB => "ONLINE_BACK",
                ClockwiseDirection::OneLineABC => "ONLINE_FRONT",
                ClockwiseDirection::OneLineACB => "ON_SEGMENT",
            };
            writeln!(write, "{ans}").ok();
        }
        write.flush().ok();
    }
}
