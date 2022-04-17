//! # $`p^e \bmod m`$
//! 繰り返し二乗法を使用して値を求める
//! ## verified by
//! [ATC002B](https://atcoder.jp/contests/atc002/submissions/26825488)

use crate::{element::integral::Integral, prelude::*};

#[snippet(name = "mod-pow", doc_hidden)]
/// # $`p^e \bmod m`$
/// ```
/// use rust_lib_for_comp::algebra::mod_pow::pow;
/// assert_eq!(130944741i64, pow(12738078907407, 9982443567 , 998244353))
/// ```
///
/// ## 計算量
/// $` \log e `$
pub fn pow<T: Integral>(mut p: T, mut e: T, m: T) -> T {
    p %= m;
    let mut res = T::one();
    while e > T::zero() {
        if e & T::one() == T::one() {
            res = res * p % m;
        }
        e >>= T::one();
        p = p * p % m;
    }
    res
}

/// # modが大きいときに使う
/// ## 計算量
/// $` \loge\logm? `$
/// ```
/// use rust_lib_for_comp::algebra::mod_pow::pow2;
/// assert_eq!(130944741i64, pow2(12738078907407, 9982443567 , 998244353))
/// ```
#[snippet(name = "mod-pow", doc_hidden)]
pub fn pow2<T: Integral>(mut p: T, mut e: T, m: T) -> T {
    fn mul<T: Integral>(mut a: T, mut b: T, m: T) -> T {
        a %= m;
        b %= m;
        let mut res = T::zero();
        while b > T::zero() {
            if b & T::one() == T::one() {
                res = (res + a) % m;
            }
            b >>= T::one();
            a = (a + a) % m;
        }
        res
    }
    p %= m;
    let mut res = T::one();
    while e > T::zero() {
        if e & T::one() == T::one() {
            res = mul(res, p, m);
        }
        e >>= T::one();
        p = mul(p, p, m);
    }
    res
}
