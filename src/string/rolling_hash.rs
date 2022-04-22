//! # ローリングハッシュ
//! 文字列の高速比較を提供する
//!
//! ## verify
//! [047 - Monochromatic Diagonal](https://atcoder.jp/contests/typical90/submissions/31161891)
use crate::algo::xor_shift::XorShift;
use crate::prelude::*;

#[snippet(name = "rolling-hash", doc_hidden)]
pub struct RollingHash {
    power: Vec<u64>,
    hash: Vec<u64>,
}

#[snippet(name = "rolling-hash", doc_hidden)]
impl RollingHash {
    const MOD: u64 = (1 << 61) - 1;
    const CHAR_MIN: u64 = 256;
    pub fn hash(&self, l: usize, r: usize) -> u64 {
        const POSITIVIZER: u64 = ((1 << 61) - 1) * 4;
        Self::calc_mod(self.hash[r] + POSITIVIZER - Self::mul(self.hash[l], self.power[r - l]))
    }
    /// a * b % MOD をオーバーフローしないよう計算する
    fn mul(a: u64, b: u64) -> u64 {
        const MASK30: u64 = (1 << 30) - 1;
        const MASK31: u64 = (1 << 31) - 1;
        let (au, ad, bu, bd) = (a >> 31, a & MASK31, b >> 31, b & MASK31);
        let mid = ad * bu + au * bd;
        let (midu, midd) = (mid >> 30, mid & MASK30);
        Self::calc_mod(au * bu * 2 + midu + (midd << 31) + ad * bd)
    }
    fn calc_mod(x: u64) -> u64 {
        const MASK61: u64 = (1 << 61) - 1;
        let (xu, xd) = (x >> 61, x & MASK61);
        match xu + xd {
            res if res >= Self::MOD => res - Self::MOD,
            res => res,
        }
    }
}

#[snippet(name = "rolling-hash", doc_hidden)]
impl From<&[char]> for RollingHash {
    fn from(src: &[char]) -> Self {
        let b = XorShift::default().rand(Self::MOD - Self::CHAR_MIN - 1) + Self::CHAR_MIN;
        let mut power = vec![1];
        let mut hash = vec![0];
        for i in 0..src.len() {
            power.push(Self::mul(power[i], b));
            hash.push(Self::calc_mod(Self::mul(hash[i], b) + src[i] as u64));
        }
        Self { power, hash }
    }
}
