//! # Miller-Rabin素数判定法
//! ここでは、$2^{64} - 1$ 以下の数について、決定的アルゴリズムとして扱う
//!
//! ## dependency
//! [mod_pow](crate::algebra::mod_pow)
//!
//! ## verify
//! [ALDS1_1_C](https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6496157)
//! [Q3. 素数判定 (強)](https://algo-method.com/submissions/394744)

use super::mod_pow::ModPow;
use crate::prelude::*;

#[snippet(name = "miller-rabin", doc_hidden)]
pub trait MillerRabin {
    /// 素数判定
    fn is_prime(&self) -> bool;
}

#[snippet(name = "miller-rabin", doc_hidden)]
impl MillerRabin for u64 {
    fn is_prime(&self) -> bool {
        /// (あるcheckerについて)nが合成数と判定されるか
        fn is_composite(n: u64, checker: u64, mut t: u64) -> bool {
            let mut x = (checker as u128).mod_pow(t as u128, n as u128);
            while t != n - 1 && x != 1 && x != n as u128 - 1 {
                x = x.mod_pow(2, n as u128);
                t <<= 1;
            }
            checker < n && t & 1 == 0 && x != n as u128 - 1
        }
        if *self < 2 || *self & 1 == 0 {
            return *self == 2; // 偶数は2だけ素数
        }
        let d = (*self - 1) >> (*self - 1).trailing_zeros(); // n-1を2で割れるだけ割ったもの
        vec![
            vec![2, 7, 61], // self < 2^32まではこっち
            vec![2, 325, 9375, 28178, 450775, 9780504, 1795265022],
        ][if *self < 1 << 32 { 0 } else { 1 }]
        .iter()
        .all(|&checker| !is_composite(*self, checker, d)) // すべてのcheckerについてすべて合成数と判定されなかった <=> selfが素数
    }
}

#[test]
fn test() {
    assert_eq!(false, 0.is_prime());
    assert_eq!(false, 1.is_prime());
    assert_eq!(true, 2.is_prime());
    assert_eq!(true, 3.is_prime());
    assert_eq!(false, 4.is_prime());
    assert_eq!(true, 5.is_prime());
    assert_eq!(false, 99.is_prime());
    assert_eq!(false, 100.is_prime());
    assert_eq!(true, 101.is_prime());
    assert_eq!(false, 1565912117761.is_prime());
}
