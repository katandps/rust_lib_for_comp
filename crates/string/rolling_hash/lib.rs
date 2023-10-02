//! # ローリングハッシュ
//! 文字列の高速比較を提供する
//!
//! ## verify
//! [047 - Monochromatic Diagonal](https://atcoder.jp/contests/typical90/submissions/31161891)
use montgomery_multiplication_64::MontgomeryReduction;
use prelude::*;
use xor_shift::XorShift;

#[snippet(name = "rolling-hash", doc_hidden)]
pub use rolling_hash_impl::{Hashed, RollingHash};
#[snippet(name = "rolling-hash", doc_hidden)]
mod rolling_hash_impl {

    use super::{MontgomeryReduction, XorShift};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Hashed {
        hash: u64,
        len: usize,
    }
    pub struct RollingHash {
        /// $b^i$のモンゴメリ表現
        power: Vec<u64>,
        /// [0, i)のHash
        hash: Vec<u64>,
    }

    impl RollingHash {
        const MOD: u64 = (1 << 61) - 1;
        const MONTGOMERY: MontgomeryReduction = MontgomeryReduction::new(Self::MOD);
        const CHAR_MIN: u64 = 256;
        /// # 部分文字列[l, r)のHashを取得
        ///
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn hash(&self, l: usize, r: usize) -> Hashed {
            assert!(l <= r);
            Hashed {
                hash: Self::MONTGOMERY.sub(
                    self.hash[r],
                    Self::MONTGOMERY.mrmul(self.power[r - l], self.hash[l]),
                ),
                len: r - l,
            }
        }

        /// # ハッシュの結合
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn concat(&self, h1: Hashed, h2: Hashed) -> Hashed {
            Hashed {
                hash: Self::MONTGOMERY
                    .add(Self::MONTGOMERY.mrmul(self.power[h2.len], h1.hash), h2.hash),
                len: h1.len + h2.len,
            }
        }

        /// # 部分文字列[l1, r1), [l2, r2)の最大共通接頭辞の長さを求める
        /// 二分探索で求める
        ///
        /// ## 計算量
        /// 短いほうの文字列の長さを$m$として、$O(\log m)$
        pub fn lcp(&self, l1: usize, r1: usize, l2: usize, r2: usize) -> usize {
            let (mut low, mut high) = (0, std::cmp::min(r1 - l1, r2 - l2) + 1);
            while high - low > 1 {
                let mid = (high + low) / 2;
                if self.hash(l1, l1 + mid) == self.hash(l2, l2 + mid) {
                    low = mid;
                } else {
                    high = mid;
                }
            }
            low
        }
    }

    impl From<&[char]> for RollingHash {
        fn from(src: &[char]) -> Self {
            let b = Self::MONTGOMERY.generate(
                XorShift::from_time().rand(Self::MOD - Self::CHAR_MIN - 1) + Self::CHAR_MIN,
            );
            let mut power = vec![Self::MONTGOMERY.generate(1)];
            let mut hash = vec![Self::MONTGOMERY.generate(0)];
            for i in 0..src.len() {
                power.push(Self::MONTGOMERY.mrmul(power[i], b));
                hash.push(Self::MONTGOMERY.add(Self::MONTGOMERY.mrmul(hash[i], b), src[i] as u64));
            }
            Self { power, hash }
        }
    }
}

#[test]
fn test() {
    let text = (0..300000)
        .map(|i| (i % 256) as u8 as char)
        .collect::<Vec<_>>();
    let rh = RollingHash::from(&text[..]);
    for i in 0..100000 {
        for j in i + 1.. {
            if rh.hash(i, i + 10000) == rh.hash(j, j + 10000) {
                assert_eq!((j - i) % 256, 0);
                break;
            }
        }
    }
}
