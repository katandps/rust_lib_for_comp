//! # リュカの定理
//! $nCr \bmod p$ を $O( p^2 \log_p N)$ で得る
use crate::prelude::*;

#[codesnip::entry("lucas-theorem", include("prelude"))]
pub fn lucas_theorem(mut n: i64, mut r: i64, p: i64) -> i64 {
    fn combination(n: i64, k: i64) -> i64 {
        if min(k, n - k) == 0 {
            1
        } else {
            combination(n - 1, k - 1) * n / k
        }
    }
    if p < 2 {
        return 0;
    }
    let mut ret = 1;
    while n != 0 || r != 0 {
        let (n_mod, r_mod) = (n % p, r % p);
        if n_mod >= r_mod {
            ret *= combination(n_mod, r_mod);
        } else {
            return 0;
        }
        n /= p;
        r /= p;
    }
    ret % p
}

#[test]
fn test() {
    assert_eq!(0, lucas_theorem(5, 2, 1));
    assert_eq!(1 % 3, lucas_theorem(5, 0, 3));
    assert_eq!(5 % 3, lucas_theorem(5, 1, 3));
    assert_eq!(10 % 3, lucas_theorem(5, 2, 3));
    assert_eq!(10 % 3, lucas_theorem(5, 3, 3));
    assert_eq!(5 % 3, lucas_theorem(5, 4, 3));
    assert_eq!(1 % 3, lucas_theorem(5, 5, 3));

    assert_eq!(1 % 3, lucas_theorem(7, 0, 3));
    assert_eq!(7 % 3, lucas_theorem(7, 1, 3));
    assert_eq!(21 % 3, lucas_theorem(7, 2, 3));
    assert_eq!(35 % 3, lucas_theorem(7, 3, 3));
    assert_eq!(35 % 3, lucas_theorem(7, 4, 3));
    assert_eq!(21 % 3, lucas_theorem(7, 5, 3));
    assert_eq!(7 % 3, lucas_theorem(7, 6, 3));
    assert_eq!(1 % 3, lucas_theorem(7, 7, 3));
}
