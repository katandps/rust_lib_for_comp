//! # $p^e \bmod m$
//! 繰り返し二乗法を使用して値を求める
//! ## verified by
//! [ATC002B](https://atcoder.jp/contests/atc002/submissions/26825488)
use algebra::*;
use prelude::*;

#[snippet(name = "mod-pow", doc_hidden)]
pub trait ModPow {
    fn mod_pow(self, exponent: Self, divisor: Self) -> Self;
    fn mod_mul(self, multiplier: Self, divisor: Self) -> Self;
}

#[snippet(name = "mod-pow", doc_hidden)]
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
        if m < T::max_value() >> T::one() {
            (self * other) % m
        } else {
            let mut res = T::zero();
            while other > T::zero() {
                if other & T::one() == T::one() {
                    res = (res + other) % m;
                }
                other >>= T::one();
                self = (self << T::one()) % m;
            }
            res
        }
    }
}
