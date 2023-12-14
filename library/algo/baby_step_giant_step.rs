//! # Baby-step Giant-step
//! $f^n(x) = y$であるような最小のnの値を求める。
//!
//! ## 引数
//! - $x$: 初期値
//! - $y$: 目標値
//! - $f(x, step)$: xにfをstep回適用する関数
//! - $f_inv(x)$: xにfを-1回適用する関数
//!
//! ## 計算量
//! $f^{-1}$の計算量を$F_{-1}$、$f^m(x)$の計算量を$F_M$とするとき
//! $\sqrt{N}max(F_{-1}, F_M)$

use crate::algo::fxhasher::HashMap;
use crate::algo::sqrt::sqrt;
use crate::prelude::*;

#[codesnip::entry("baby-step-giant-step", include("faster-hashmap", "sqrt", "prelude"))]
pub fn baby_step_giant_step<T, F, FINV>(
    mut x: T,
    y: T,
    max: i64,
    mut f: F,
    mut f_inv: FINV,
) -> Option<i64>
where
    T: Copy + Eq + Hash,
    F: FnMut(T, i64) -> T,
    FINV: FnMut(T) -> T,
{
    let p = sqrt(max).0;
    let mut candidates = HashMap::default();
    // 目標値までi回となるような値vを計算しておく → vにk回で到達できるとき、k+i回でゴールに到達できる
    let mut cur = y;
    for i in 0..=p {
        candidates.entry(cur).or_insert(i);
        cur = f_inv(cur);
    }
    // $p$ 回ずつまとめてxを遷移させる → p回以内に到達できる点があれば
    for i in 0..=max / p {
        if let Some(v) = candidates.get(&x) {
            return Some(i * p + v);
        }
        x = f(x, p);
    }
    None
}

#[test]
fn test() {
    use crate::algebra::mod_pow::ModPow;
    const MOD: i64 = 10007;
    let x = 2;
    let mut cur = 1;
    let mut v = vec![None; MOD as usize];
    for i in 0..MOD {
        if v[cur as usize].is_none() {
            v[cur as usize] = Some(i);
        }
        cur *= x;
        cur %= MOD;
    }

    let result = (0..MOD)
        .map(|y| {
            baby_step_giant_step(
                1,
                y,
                MOD,
                |x, step| x * 2.mod_pow(step, MOD) % MOD,
                |x| x * 2.mod_pow(MOD - 2, MOD) % MOD,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(v, result);
}
