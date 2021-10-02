//! 最長増加部分列 (Longest Increasing Subsequence)

use std::collections::VecDeque;

/// $`長さ L の数列 A の最長増加部分列とは、
/// 1 \leq i_1 < i_2 < ... < i_M \leq L
/// かつ A_{i_1} < A_{i_2} < ... < A_{i_M} を満たす部分列 A_{i_1} , A_{i_2} , ... , A_{i_M} の中で、
/// 最も M が大きいもの のこと`$

const INF: i64 = 1 << 60;

#[derive(Debug, Clone)]
pub struct LIS {
    n: usize,
    dp: Vec<i64>,
    stack: VecDeque<(usize, i64)>,
}

impl LIS {
    pub fn new(n: usize) -> LIS {
        LIS {
            n,
            dp: vec![INF; n],
            stack: VecDeque::new(),
        }
    }

    /// LISを更新する
    pub fn insert(&mut self, a: i64) {
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
        self.stack.push_front((ok as usize, self.dp[ok as usize]));
        self.dp[ok as usize] = a;
    }

    /// 最長増加部分列の長さを返す
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

    /// 更新をひとつ分巻き戻す
    pub fn rollback(&mut self) {
        if let Some((pos, val)) = self.stack.pop_front() {
            self.dp[pos] = val;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut lis = LIS::new(5);
        let v = vec![1, 4, 2, 3, 5];
        let expect = vec![1, 2, 2, 3, 4];
        for i in 0..5 {
            lis.insert(v[i]);
            assert_eq!(expect[i], lis.calc());
        }

        for i in (0..5).rev() {
            assert_eq!(expect[i], lis.calc());
            lis.rollback();
        }
    }
}
