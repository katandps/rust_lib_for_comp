//! # Minimum Cost Flow(最小費用流)

use rust_lib_for_comp::flow::primal_dual::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Grl6B;
impl verify::Solver for Grl6B {
    const PROBLEM_ID: &'static str = "GRL_6_B";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (v, e, f) = reader.v3::<usize, usize, i64>();
        let uvcd = reader.vec4::<usize, usize, i64, i64>(e);

        let mut pd = PrimalDual::new(v);
        for (u, v, c, d) in uvcd {
            pd.add_edge(u, v, c, d);
        }
        if let Some(ans) = pd.min_cost_flow(0, v - 1, f) {
            writeln!(write, "{ans}").ok();
        } else {
            writeln!(write, "-1").ok();
        }
        write.flush().ok();
    }
}
