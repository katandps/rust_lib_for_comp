//! # ローリングハッシュ
//! 文字列の高速比較を提供する
use crate::algo::xor_shift::XorShift;
use crate::prelude::*;

#[snippet(name = "rolling-hash", doc_hidden)]
pub struct RollingHash {
    power: Vec<i128>,
    hash: Vec<i128>,
}

#[snippet(name = "rolling-hash", doc_hidden)]
impl RollingHash {
    const MOD: i128 = (1 << 61) - 1;
    const CHAR_MIN: i128 = 256;

    /// # [l, r)のハッシュ値
    pub fn hash(&self, l: usize, r: usize) -> u64 {
        let hash = self.hash[r] - (self.hash[l] * self.power[r - l] % Self::MOD);
        if hash > 0 {
            hash as u64
        } else {
            (hash + Self::MOD) as u64
        }
    }
}

#[snippet(name = "rolling-hash", doc_hidden)]
impl From<&[char]> for RollingHash {
    fn from(src: &[char]) -> Self {
        let mut randomizer = XorShift::default();
        let b = (randomizer.rand((Self::MOD - Self::CHAR_MIN - 1) as u64) + (1 << 8)) as i128; // charの最大値..MOD-1;
        let mut power = vec![1];
        for i in 0..src.len() {
            power.push((power[i] * b) % Self::MOD);
        }
        let mut hash = vec![0];
        for i in 0..src.len() {
            hash.push((hash[i] * b + src[i] as i128) % Self::MOD);
        }

        Self { power, hash }
    }
}
