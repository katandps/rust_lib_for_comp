//! # ローリングハッシュ
//! 文字列の高速比較を提供する
//!
//! ## verify
//! [047 - Monochromatic Diagonal](https://atcoder.jp/contests/typical90/submissions/31161891)
use algebra::Zero;
use mod_int_64bit::ModInt64;
use prelude::*;
use xor_shift::XorShift;

#[snippet(name = "rolling-hash", doc_hidden)]
pub use rolling_hash_impl::RollingHash;
#[snippet(name = "rolling-hash", doc_hidden)]
mod rolling_hash_impl {
    use super::{Add, Debug, ModInt64, Sub, XorShift, Zero};
    use std::sync::OnceLock;

    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct RollingHash {
        pub hash: Hash,
        pub len: usize,
    }
    type Hash = ModInt64<{ (1 << 61) - 1 }>;
    static BASE: OnceLock<Hash> = OnceLock::new();
    static LENGTH: usize = 1000100;
    static BASE_POW: OnceLock<Vec<Hash>> = OnceLock::new();

    const CHAR_MAX: u64 = 256; // 文字の最大値
    #[inline]
    fn get_base() -> Hash {
        BASE.get_or_init(|| Hash::from(10000));
        *BASE.get_or_init(|| {
            Hash::from(XorShift::from_time().rand(Hash::MOD - CHAR_MAX - 1) + CHAR_MAX)
        })
    }

    #[inline]
    fn get_pow(e: usize) -> Hash {
        BASE_POW.get_or_init(|| {
            let mut v = Vec::with_capacity(LENGTH + 1);
            let base = get_base();
            v.push(ModInt64::one());
            for i in 0..LENGTH {
                v.push(v[i] * base)
            }
            v
        })[e]
    }

    impl RollingHash {
        pub fn new(value: i64) -> Self {
            RollingHash {
                hash: ModInt64::from(value),
                len: 1,
            }
        }
    }

    impl From<char> for RollingHash {
        fn from(value: char) -> Self {
            RollingHash {
                hash: ModInt64::from(value as u32),
                len: 1,
            }
        }
    }

    /// # 加算
    ///
    /// "ab" + "cd" = "abcd"
    impl Add<Self> for RollingHash {
        type Output = Self;
        // "ab" + "cd" = "abcd"
        fn add(self, rhs: Self) -> Self {
            Self {
                hash: self.hash + rhs.hash * get_pow(self.len),
                len: self.len + rhs.len,
            }
        }
    }
    /// # 減算
    ///
    /// "abcd" - "ab" = "cd"
    ///
    /// 結果に対応する文字列が存在するとは限らない
    impl Sub<Self> for RollingHash {
        type Output = Self;
        // "abcd" - "ab" = "cd"
        fn sub(self, rhs: Self) -> Self::Output {
            debug_assert!(self.len >= rhs.len);
            Self {
                hash: (self.hash - rhs.hash) / get_pow(rhs.len),
                len: self.len - rhs.len,
            }
        }
    }

    impl Zero for RollingHash {
        fn zero() -> Self {
            Self {
                hash: Hash::zero(),
                len: 0,
            }
        }
    }

    impl Debug for RollingHash {
        fn fmt(&self, f: &mut prelude::Formatter<'_>) -> std::fmt::Result {
            write!(f, "hash:{}, len:{}", self.hash, self.len)
        }
    }
}

#[test]
fn test() {
    const CYCLE: usize = 256;
    let hash = (0usize..3000)
        .map(|i| RollingHash::new((i % CYCLE) as i64))
        .collect::<Vec<_>>();
    let mut sum = vec![RollingHash::default()];
    for i in 0..hash.len() {
        sum.push(sum[i] + hash[i]);
    }
    for i in 0..1000 {
        for j in i + 1.. {
            if sum[i + 100] - sum[i] == sum[j + 100] - sum[j] {
                assert_eq!((j - i) % CYCLE, 0);
                break;
            } else {
                assert_ne!((j - i) % CYCLE, 0, "{} {}", i, j)
            }
        }
    }
}

#[test]
fn add_sub_test() {
    let hash1 = RollingHash::new(1);
    let hash2 = RollingHash::new(2);
    let hash3 = RollingHash::new(3);
    let hash4 = RollingHash::new(4);
    assert_eq!(
        hash1 + hash2 + hash3 + hash4,
        hash1 + (hash2 + hash3) + hash4
    );
    assert_eq!(
        hash1 + hash2 + hash3 + hash4,
        (hash1 + hash2) + (hash3 + hash4)
    );
    assert_eq!(hash2 + hash3 + hash4, hash1 + hash2 + hash3 + hash4 - hash1);
}
