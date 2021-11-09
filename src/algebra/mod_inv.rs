//! 剰余体における逆元

use crate::prelude::*;

#[snippet(name = "mod-inv", doc_hidden)]
pub struct ModInv;

#[snippet(name = "mod-inv", doc_hidden)]
impl ModInv {
    ///
    /// numberの逆元をmod moduloで求める
    /// ```
    /// use rust_competitive_programming::algebra::mod_inv::ModInv;
    /// assert_eq!(1, ModInv::inv(1, 13));
    /// assert_eq!(7, ModInv::inv(2, 13));
    /// assert_eq!(9, ModInv::inv(3, 13));
    /// assert_eq!(10, ModInv::inv(4, 13));
    /// assert_eq!(8, ModInv::inv(5, 13));
    /// assert_eq!(11, ModInv::inv(6, 13));
    /// assert_eq!(2, ModInv::inv(7, 13));
    /// assert_eq!(5, ModInv::inv(8, 13));
    /// assert_eq!(3, ModInv::inv(9, 13));
    /// assert_eq!(4, ModInv::inv(10, 13));
    /// assert_eq!(6, ModInv::inv(11, 13));
    /// assert_eq!(12, ModInv::inv(12, 13));
    /// assert_eq!(0, ModInv::inv(13, 13));
    /// assert_eq!(1, ModInv::inv(14, 13));
    /// ```
    pub fn inv(mut number: i64, modulo: i64) -> i64 {
        let (mut b, mut u, mut v) = (modulo, 1, 0);
        while b > 0 {
            let t = number / b;
            number -= t * b;
            swap(&mut number, &mut b);
            u -= t * v;
            swap(&mut u, &mut v);
        }
        u %= modulo;
        if u < 0 {
            u += modulo;
        }
        u
    }
}
