#[allow(unused_imports)]
use longest_increasing_subsequence::*;

#[allow(dead_code)]
pub mod longest_increasing_subsequence {
    type VALUE = usize;
    const INF: usize = 1 << 60;
    /// 最長増加部分列を求める O(NlogN)
    pub fn lis(v: &Vec<VALUE>) -> usize {
        let n = v.len();
        let mut dp = vec![INF; n];
        for i in 0..n {
            let ai = v[i];
            let mut ok = n as i32;
            let mut ng = -1;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                // ok は dp[ok] >= ai となる最小のindex
                if dp[mid as usize] >= ai {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            dp[ok as usize] = ai;
        }
        let mut ok = n as i32;
        let mut ng = -1;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if dp[mid as usize] >= INF {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}
