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
    fn is_prime(&self) -> bool;
    fn is_composite(&self, checker: u64, n_1: u64) -> bool;
}

#[snippet(name = "miller-rabin", doc_hidden)]
impl MillerRabin for u64 {
    fn is_prime(&self) -> bool {
        if *self < 2 || *self & 1 == 0 {
            return *self == 2; // 偶数は2だけ素数
        }
        let d = (*self - 1) >> (*self - 1).trailing_zeros(); // n-1を2で割れるだけ割ったもの
        vec![
            vec![2, 7, 61], // self < 2^32まではこっち
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37],
        ][if *self < 1 << 32 { 0 } else { 1 }]
        .iter()
        .all(|&checker| !self.is_composite(checker, d)) // すべてのcheckerについてすべて合成数と判定されなかった <=> selfが素数
    }
    fn is_composite(&self, a: u64, mut t: u64) -> bool {
        let mut x = (a as u128).mod_pow(t as u128, *self as u128);
        while t != *self - 1 && x != 1 && x != *self as u128 - 1 {
            x = x.mod_pow(2, *self as u128);
            t <<= 1;
        }
        a < *self && t & 1 == 0 && x != *self as u128 - 1
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
