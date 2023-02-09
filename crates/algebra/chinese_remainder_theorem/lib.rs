//! # 中国剰余定理
//! $x \equiv b1 \bmod m1$, $x \equiv b2 \bmod m2$ となる整数$x$, $m$を得る $(x \equiv r \bmod m)$
//! ```
//! # use chinese_remainder_theorem::*;
//!
//! fn assert(b1: u64, m1: u64, b2: u64, m2: u64) {
//!     let (r, m) = CRT::crt(b1, m1, b2, m2);
//!     assert_eq!(r % m1, b1);
//!     assert_eq!(r % m2, b2);
//!     assert_eq!(0, m % m1);
//!     assert_eq!(0, m % m2)
//! }
//! assert(2, 3, 3, 5);
//! assert(3, 16, 0, 45);
//! assert(4, 10, 2, 16);
//! assert(1, 1_000_000_000_000_001, 0, 2);
//! ```
use prelude::*;

#[snippet(name = "chinese-remainder-theorem", doc_hidden)]
pub struct CRT;

#[snippet(name = "chinese-remainder-theorem", doc_hidden)]
impl CRT {
    /// (r, m) の順で返却
    /// 値がない場合は(0,0)を返す
    pub fn crt(b1: u64, m1: u64, b2: u64, m2: u64) -> (u64, u64) {
        let (b1, b2, m1, m2) = (b1 as i128, b2 as i128, m1 as i128, m2 as i128);
        let (d, p, _q) = Self::ext_gcd(m1, m2);
        if (b2 - b1) % d != 0 {
            (0, 0)
        } else {
            let m = m1 * (m2 / d); //lcm
            let tmp = (b2 - b1) / d * p % (m2 / d);
            let r = Self::mo(b1 + m1 * tmp, m);
            (r as u64, m as u64)
        }
    }

    ///拡張Euclidの互除法 返り値 (gcd(a,b), p, q)
    /// (p,q) は ap + bq = gcd(a, b) となるp, q
    fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (gcd, q, p) = Self::ext_gcd(b, a % b);
            let q = q - (a / b) * p;
            (gcd, p, q)
        }
    }

    fn mo(a: i128, m: i128) -> i128 {
        (a % m + m) % m
    }
}

#[test]
fn ext_gcd_test() {
    let (a, b) = (10, 34);
    let (d, p, q) = CRT::ext_gcd(a, b);
    assert_eq!(2, d);
    assert_eq!(a * p + b * q, d);
}
