//! # [Hard Beans](https://onlinejudge.u-aizu.ac.jp/problems/1549)

use rust_lib_for_comp::data_structure::wavelet_matrix::WaveletMatrix;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct P1549;
impl verify::Solver for P1549 {
    const PROBLEM_ID: &'static str = "1549";
    const TIME_LIMIT_MILLIS: u64 = 2000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let a = reader.vec::<u64>(n);
        let wm = WaveletMatrix::from(a);
        for _ in 0..reader.v() {
            let (l, r, d) = reader.v3::<usize, usize, u64>();
            let prev = wm.prev(l..=r, d);
            let next = wm.next(l..=r, d);
            let ans = match (prev, next) {
                (Some(prev), Some(next)) => std::cmp::min(d - prev, next - d),
                (Some(prev), _) => d - prev,
                (_, Some(next)) => next - d,
                _ => 0,
            };
            writeln!(write, "{ans}").ok();
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    P1549::assert(
        "3
    1 2 3
    3
    0 2 2
    0 2 4
    0 0 2",
        "0
        1
        1",
    );
    P1549::assert(
        "10
    4 5 0 21 9 100 12 9 0 8
    5
    0 3 20
    2 5 100
    8 9 9
    5 5 10
    0 9 20",
        "1
        0
        1
        90
        1",
    )
}
