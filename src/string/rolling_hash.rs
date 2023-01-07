//! # ローリングハッシュ
//! 文字列の高速比較を提供する
//!
//! ## verify
//! [047 - Monochromatic Diagonal](https://atcoder.jp/contests/typical90/submissions/31161891)
use crate::algo::xor_shift::XorShift;
use crate::prelude::*;

#[snippet(name = "rolling-hash", doc_hidden)]
pub use rolling_hash_impl::{Hashed, RollingHash};
#[snippet(name = "rolling-hash", doc_hidden)]
mod rolling_hash_impl {
    use super::XorShift;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Hashed {
        hash: u64,
        len: usize,
    }
    pub struct RollingHash {
        power: Vec<u64>,
        hash: Vec<u64>,
    }

    impl RollingHash {
        const MOD: u64 = (1 << 61) - 1;
        const CHAR_MIN: u64 = 256;
        /// # 部分文字列[l, r)のHashを取得
        ///
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn hash(&self, l: usize, r: usize) -> Hashed {
            assert!(l <= r);
            const POSITIVIZER: u64 = ((1 << 61) - 1) * 4;
            Hashed {
                hash: Self::calc_mod(
                    self.hash[r] + POSITIVIZER - Self::mul(self.hash[l], self.power[r - l]),
                ),
                len: r - l,
            }
        }

        /// # 文字列の結合
        /// 長さとhashがわかっていれば、結合できる
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn concat(&self, h1: Hashed, h2: Hashed) -> Hashed {
            Hashed {
                hash: (Self::mul(self.power[h2.len], h1.hash) + h2.hash) % Self::MOD,
                len: h1.len + h2.len,
            }
        }

        /// # 部分文字列[l1, r1), [l2, r2)の最大共通接頭辞を求める
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
}

#[test]
fn test() {
    let text = (0..3000000)
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
