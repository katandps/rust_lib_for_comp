//! # $`p^e \bmod m`$
//! ## 計算量
//! $` \log e`$
//! ## verified by
//! [ATC002B](https://atcoder.jp/contests/atc002/submissions/26825488)

use crate::prelude::*;

#[snippet(name = "mod-pow", doc_hidden)]
/// # $`p^e \bmod m`$
/// ```
/// use rust_lib_for_comp::algebra::mod_pow::pow;
/// assert_eq!(130944741, pow(12738078907407, 9982443567 , 998244353))
/// ```
pub fn pow(mut p: i64, mut e: i64, m: i64) -> i64 {
    p %= m;
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 {
            res = res * p % m;
        }
        e >>= 1;
        p = p * p % m;
    }
    res
}
