//! # $`p^e \bmod m`$
//! ## 計算量
//! $` \log e`$
//! ## verified by
//! [ATC002B](https://atcoder.jp/contests/atc002/submissions/26825488)

use crate::{element::integral::Integral, prelude::*};

#[snippet(name = "mod-pow", doc_hidden)]
/// # $`p^e \bmod m`$
/// ```
/// use rust_lib_for_comp::algebra::mod_pow::pow;
/// assert_eq!(130944741i64, pow(12738078907407, 9982443567 , 998244353))
/// ```
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
