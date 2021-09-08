#[allow(unused_imports)]
use floor_sum::*;

#[allow(dead_code)]
mod floor_sum {
    type NUM = i64;

    /// y = (Ax + B)/M (0 <= x <= N) の下にある格子点の数を求める
    /// O(log(max(A,M))
    pub fn floor_sum(n: NUM, m: NUM, mut a: NUM, mut b: NUM) -> NUM {
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
}
