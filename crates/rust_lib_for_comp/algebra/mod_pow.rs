//! # $p^e \bmod m$
//! 繰り返し二乗法を使用して値を求める
//! ## verified by
//! [ATC002B](https://atcoder.jp/contests/atc002/submissions/26825488)
use crate::algebra::*;

#[codesnip::entry("mod-pow", include("algebra"))]
pub trait ModPow {
    fn mod_pow(self, exponent: Self, divisor: Self) -> Self;
    fn mod_mul(self, multiplier: Self, divisor: Self) -> Self;
}

#[codesnip::entry("mod-pow", include("algebra"))]
impl<T: Integral> ModPow for T {
    fn mod_pow(mut self, mut e: T, m: T) -> T {
        self %= m;
        let mut res = T::one();
        while e > T::zero() {
            if e & T::one() == T::one() {
                res = res.mod_mul(self, m);
            }
            e >>= T::one();
            self = self.mod_mul(self, m);
        }
        res
    }
    fn mod_mul(mut self, mut other: T, m: T) -> T {
        self %= m;
        other %= m;
        if self <= T::max_value() / other {
            (self * other) % m
        } else {
            if other > self {
                swap(&mut self, &mut other);
            }
            let mut res = T::zero();
            while other > T::zero() {
                if other & T::one() == T::one() {
                    if T::max_value() - res < self {
                        // overflow
                        res = (T::max_value() % m + (self - (T::max_value() - res))) % m;
                    } else {
                        res = (res + self) % m;
                    }
                }
                other >>= T::one();
                if self << T::one() < self {
                    // overflow
                    self = ((self << T::one()) % m + T::max_value() % m + T::one()) % m;
                } else {
                    self = (self << T::one()) % m;
                }
            }
            res
        }
    }
}

#[test]
fn test() {
    assert_eq!(16.mod_mul(16, 200), 56);
    assert_eq!(16u8.mod_mul(16, 200), 56);
    assert_eq!(16.mod_pow(2, 200), 56);
    assert_eq!(16u8.mod_pow(2, 200), 56);
}
