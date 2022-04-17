//! # XorShift法による疑似乱数生成
use crate::prelude::*;

#[snippet(name = "xor-shift", doc_hidden)]
#[derive(Debug)]
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
    pub fn with_seed(seed: u64) -> Self {
        Self { seed }
    }
    pub fn rand(&mut self, m: u64) -> u64 {
        self.next().unwrap() % m
    }
    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0000000000000;
        const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
        f64::from_bits(UPPER_MASK | (self.next().unwrap() & LOWER_MASK)) - 1.0
    }
}
