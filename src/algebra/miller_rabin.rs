//! # Miller-Rabin素数判定法
//! ここでは、$`2^64 - 1`$ 以下の数について、決定的アルゴリズムとして扱う
//!
//! ## dependency
//! [mod_pow](crate::algebra::mod_pow)
//!
//! ## verify
//! [ALDS1_1_C](https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6496119#2)

use super::mod_pow::pow;
use crate::prelude::*;

#[snippet(name = "miller-rabin", doc_hidden)]
pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n & 1 == 0 {
        return false;
    }
    let mut d = n - 1;
    d >>= d.trailing_zeros();
    fn suspect(a: u64, mut t: u64, n: u64) -> bool {
        let mut x = pow(a, t, n);
        while t != n - 1 && x != 1 && x != n - 1 {
            x = pow(x, 2, n);
            t <<= 1;
        }
        t & 1 == 1 || x == n - 1
    }
    if n < 1 << 32 {
        const LIST: [u64; 3] = [2, 7, 61];
        if LIST.iter().filter(|&&k| k < n && !suspect(k, d, n)).count() > 0 {
            return false;
        }
    } else {
        const LIST: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        if LIST.iter().filter(|&&k| k < n && !suspect(k, d, n)).count() > 0 {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    assert_eq!(false, is_prime(0));
    assert_eq!(false, is_prime(1));
    assert_eq!(true, is_prime(2));
    assert_eq!(true, is_prime(3));
    assert_eq!(false, is_prime(4));
    assert_eq!(true, is_prime(5));
    assert_eq!(false, is_prime(99));
    assert_eq!(false, is_prime(100));
    assert_eq!(true, is_prime(101));
}
