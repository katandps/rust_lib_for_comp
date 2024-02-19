//! # Maximum Flow(最大流)

use rust_lib_for_comp::flow::dinic::*;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct Grl6A;
impl verify::Solver for Grl6A {
    const PROBLEM_ID: &'static str = "GRL_6_A";
    const EPSILON: Option<f64> = Some(1e-5);
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (v, e) = reader.v2::<usize, usize>();
        let uvc = reader.vec3::<usize, usize, i64>(e);
        let mut dinic = Dinic::new(v);
        for (u, v, c) in uvc {
            dinic.add_edge(u, v, c);
        }
        let ans = dinic.max_flow(0, v - 1);
        writeln!(write, "{ans}").ok();
        write.flush().ok();
    }
}

#[test]
fn test() {
    Grl6A::assert(
        "4 5
    0 1 2
    0 2 1
    1 2 1
    1 3 1
    2 3 2",
        "3",
    )
}
