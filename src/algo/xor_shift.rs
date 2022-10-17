//! # XorShift法による疑似乱数生成
use crate::prelude::*;

#[snippet(name = "xor-shift", doc_hidden)]
#[derive(Clone, Debug)]
pub struct XorShift {
    seed: u64,
}

#[snippet(name = "xor-shift", doc_hidden)]
impl Default for XorShift {
    fn default() -> Self {
        let seed = 0xf0fb588ca2196dac;
        Self { seed }
    }
}

#[snippet(name = "xor-shift", doc_hidden)]
impl Iterator for XorShift {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        Some(self.seed)
    }
}

#[snippet(name = "xor-shift", doc_hidden)]
impl XorShift {
    /// # シードを指定して初期化
    pub fn with_seed(seed: u64) -> Self {
        Self { seed }
    }
    /// # 乱数を生成
    /// 0..mの範囲で乱数を生成する
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next().unwrap() % m
    }
    /// # 範囲指定して乱数を生成
    /// rangeの範囲で乱数を生成する
    pub fn rand_range<R: RangeBounds<i64>>(&mut self, range: R) -> i64 {
        let (l, r) = range.to_lr();
        let k = self.next().unwrap() as i64;
        k.rem_euclid(r - l) + l
    }
    /// # 浮動小数点数の乱数を生成
    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        f64::from_bits(UPPER_MASK | (self.next().unwrap() & LOWER_MASK)) - 1.0
    }
}
