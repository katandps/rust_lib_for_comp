//! # [Awkward Lights](https://onlinejudge.u-aizu.ac.jp/problems/1308)

use rust_lib_for_comp::data_structure::bit_matrix::BitMatrix;
use rust_lib_for_comp::util::io_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct P1308;
impl verify::Solver for P1308 {
    const PROBLEM_ID: &'static str = "1308";
    const TIME_LIMIT_MILLIS: u64 = 8000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        loop {
            let (m, n, d) = reader.v3::<usize, usize, usize>();
            if m == 0 && n == 0 && d == 0 {
                break;
            }
            let s = reader.matrix::<usize>(n, m);
            let mut bm = BitMatrix::new(n * m, n * m);
            let mut vec = vec![false; n * m];
            for i in 0..n {
                for j in 0..m {
                    bm.set(i * m + j, i * m + j, true);
                    for k in 0..n {
                        for l in 0..m {
                            if ((i as i64 - k as i64).abs() + (j as i64 - l as i64).abs())
                                != d as i64
                            {
                                continue;
                            }
                            bm.set(i * m + j, k * m + l, true);
                        }
                    }
                    vec[i * m + j] = true ^ (s[i][j] > 0);
                }
            }
            let result = bm.linear_equation(&vec);
            writeln!(write, "{}", if result.is_some() { 1 } else { 0 }).ok();
        }
        write.flush().ok();
    }
}

#[test]
fn test() {
    P1308::assert(
        "1 1 1
    1
    2 2 1
    1 1
    1 1
    3 2 1
    1 0 1
    0 1 0
    3 3 1
    1 0 1
    0 1 0
    1 0 1
    4 4 2
    1 1 0 1
    0 0 0 1
    1 0 1 1
    1 0 0 0
    5 5 1
    1 1 1 0 1
    0 1 0 1 0
    1 0 1 0 1
    0 1 0 1 0
    1 0 1 0 1
    5 5 2
    0 0 0 0 0
    0 0 0 0 0
    0 0 1 0 0
    0 0 0 0 0
    0 0 0 0 0
    11 11 3
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 1 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    11 11 3
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 1 1 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0
    13 13 7
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 1 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0 0 0 0 0 0
    0 0 0",
        "1
        1
        0
        1
        0
        0
        1
        1
        0
        1",
    )
}
