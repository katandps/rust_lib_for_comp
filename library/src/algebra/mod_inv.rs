//! 剰余体における逆元

#[allow(unused_imports)]
use mod_inv::*;

#[allow(dead_code)]
pub mod mod_inv {
    use std::mem::swap;

    ///
    /// numberの逆元をmod moduloで求める
    /// ```
    /// use library::algebra::mod_inv::mod_inv::mod_inv;
    /// assert_eq!(1, mod_inv(1, 13));
    /// assert_eq!(7, mod_inv(2, 13));
    /// assert_eq!(9, mod_inv(3, 13));
    /// assert_eq!(10, mod_inv(4, 13));
    /// assert_eq!(8, mod_inv(5, 13));
    /// assert_eq!(11, mod_inv(6, 13));
    /// assert_eq!(2, mod_inv(7, 13));
    /// assert_eq!(5, mod_inv(8, 13));
    /// assert_eq!(3, mod_inv(9, 13));
    /// assert_eq!(4, mod_inv(10, 13));
    /// assert_eq!(6, mod_inv(11, 13));
    /// assert_eq!(12, mod_inv(12, 13));
    /// assert_eq!(0, mod_inv(13, 13));
    /// assert_eq!(1, mod_inv(14, 13));
    /// ```
    pub fn mod_inv(number: isize, modulo: usize) -> usize {
        let mut n = number;
        let mut b = modulo as isize;
        let mut u: isize = 1;
        let mut v: isize = 0;

        while b > 0 {
            let t: isize = n / b;
            n -= t * b;
            swap(&mut n, &mut b);
            u -= t * v;
            swap(&mut u, &mut v);
        }
        u %= modulo as isize;
        if u < 0 {
            u += modulo as isize;
        }
        u as usize
    }
}
