//! 最長増加部分列を求める (Longest Increasing Subsequence)
//! $`長さ L の数列 A の最長増加部分列とは、
//! 1 \leq i_1 < i_2 < ... < i_M \leq L
//! かつ A_{i_1} < A_{i_2} < ... < A_{i_M} を満たす部分列 A_{i_1} , A_{i_2} , ... , A_{i_M} の中で、
//! 最も M が大きいもの のこと`$

#[allow(unused_imports)]
use longest_increasing_subsequence::*;

#[allow(dead_code)]
pub mod longest_increasing_subsequence {
    const INF: i64 = 1 << 60;

    #[derive(Debug, Clone)]
    pub struct LIS {
        n: usize,
        dp: Vec<i64>,
    }

    impl LIS {
        pub fn new(n: usize) -> LIS {
            LIS {
                n,
                dp: vec![INF; n],
            }
        }

        /// LISを更新する
        /// 更新したものの位置と値を返す
        pub fn insert(&mut self, a: i64) -> (usize, i64) {
            let mut ok = self.n as i64;
            let mut ng = -1;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if self.dp[mid as usize] >= a {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            let ret = (ok as usize, self.dp[ok as usize]);
            self.dp[ok as usize] = a;
            ret
        }

        pub fn calc(&self) -> usize {
            let mut ok = 0;
            let mut ng = self.n as i64;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if self.dp[mid as usize] < INF {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok as usize + 1
        }

        pub fn rollback(&mut self, rollback: (usize, i64)) {
            self.dp[rollback.0] = rollback.1;
        }
    }
}
