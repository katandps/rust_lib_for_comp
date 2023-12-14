//! # XorShift法による疑似乱数生成
use prelude::*;
use range_traits::*;

#[snippet(name = "xor-shift", doc_hidden)]
#[derive(Clone, Debug)]
pub struct XorShift {
    seed: u64,
}

#[snippet(name = "xor-shift", doc_hidden)]
mod xor_shift_impl {
    use std::time::SystemTime;

    use super::{ToBounds, XorShift};

    const DEFAULT_SEED: u64 = 0xf0fb588ca2196dac;
    impl Default for XorShift {
        #[inline]
        fn default() -> Self {
            Self { seed: DEFAULT_SEED }
        }
    }

    impl Iterator for XorShift {
        type Item = u64;
        #[inline]
        fn next(&mut self) -> Option<u64> {
            self.seed ^= self.seed << 13;
            self.seed ^= self.seed >> 7;
            self.seed ^= self.seed << 17;
            Some(self.seed)
        }
    }

    impl XorShift {
        pub fn from_time() -> Self {
            let mut ret = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(elapsed) => Self {
                    seed: elapsed.as_millis() as u64,
                },
                Err(e) => {
                    panic!("{}", e);
                }
            };
            for _ in 0..40 {
                ret.next();
            }
            ret
        }

        /// # シードを指定して初期化
        pub fn with_seed(seed: u64) -> Self {
            Self { seed }
        }
        /// # 乱数を生成
        /// 0..mの範囲で乱数を生成する
        #[inline]
        pub fn rand(&mut self, m: u64) -> u64 {
            self.next().unwrap() % m
        }
        /// # 範囲指定して乱数を生成
        /// rangeの範囲で乱数を生成する
        #[inline]
        pub fn rand_range<R: ToBounds<i64>>(&mut self, range: R) -> i64 {
            let (l, r) = range.lr();
            let k = self.next().unwrap() as i64;
            k.rem_euclid(r - l) + l
        }
        /// # 浮動小数点数の乱数を生成
        #[inline]
        pub fn randf(&mut self) -> f64 {
            const UPPER_MASK: u64 = 0x3FF0000000000000;
            const LOWER_MASK: u64 = 0xFFFFFFFFFFFFF;
            f64::from_bits(UPPER_MASK | (self.next().unwrap() & LOWER_MASK)) - 1.0
        }

        #[inline]
        pub fn shuffle<T>(&mut self, s: &mut [T]) {
            for i in (1..s.len()).rev() {
                s.swap(i, self.rand_range(0..i as i64) as usize);
            }
        }
    }
}
