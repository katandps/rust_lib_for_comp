//! # ラグランジュ補間
//! n次の多項式の補間を求める
//!
//! ## 計算量
//! $O(n\log(n)^2)$
//!
//! ## todo
//! 0..=n以外の値を与えたときの機能
use const_mod_val_table::ModValTable;
use mod_int::ModInt;
use prelude::*;

/// # ラグランジュ補間
/// $N$次の多項式関数 $f(x)$ について、$f(0)$, $f(1)$, ... $f(n)$ を満たす
/// $f(t)$を求める
#[snippet(name = "lagrange-polynomical", doc_hidden)]
pub fn lagrange_polynomical<const N: usize, const M: u32>(
    mvt: &ModValTable<M, N>,
    v: &[ModInt<M>],
    t: usize,
) -> ModInt<M> {
    let n = v.len() - 1;
    if t <= n {
        return v[t];
    }
    let mut ret = ModInt::zero();
    let mut dp = vec![ModInt::one(); n + 1];
    let mut pd = vec![ModInt::one(); n + 1];
    for i in 0..n {
        dp[i + 1] = dp[i] * (t - i);
    }
    for i in (1..=n).rev() {
        pd[i - 1] = pd[i] * (t - i);
    }
    for i in 0..=n {
        ret += v[i] * dp[i] * pd[i] / mvt.factorial(i as i64) / mvt.factorial((n - i) as i64)
            * if (n - i) & 1 == 1 { -1 } else { 1 };
    }
    ret
}
