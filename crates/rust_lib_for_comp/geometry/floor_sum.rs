//! # 直線と格子点
//! $y = \frac{Ax + B}{M} (0 \leq x \leq N) $  の下にある格子点の数を求める
//! ## 計算量
//! $O(\log \max_{A, M})$

#[codesnip::entry("floor-sum")]
pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0;
    if a >= m {
        ans += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }
    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;
    if y_max != 0 {
        ans += (n - (x_max + a - 1) / a) * y_max;
        ans += floor_sum(y_max, a, m, (a - x_max % a) % a);
    }
    ans
}
