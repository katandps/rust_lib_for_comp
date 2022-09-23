//! # モンゴメリ乗算
//!
//!

use crate::prelude::*;

#[snippet(name = "montgomery-multiplication", doc_hidden)]
#[derive(Clone, Debug)]
pub struct MontgomeryU64 {
    /// 奇数$N$
    pub n: u64,
    /// $n*ni == 1 \mod 2^64$
    pub ni: u64,
    /// $nh = \frac{n+1}{2}$
    pub nh: u64,
    /// $2^64 \mod N$
    pub r: u64,
    /// $-(2^64) \mod N)
    pub rn: u64,
    /// $(2^64)^2 \mod N)
    pub r2: u64,
    /// (n-1) >> (n-1).trailing_zeros()
    pub d: u64,
    /// (n-1).trailing_zeros()
    pub k: u32,
}

#[snippet(name = "montgomery-multiplication", doc_hidden)]
impl MontgomeryU64 {
    #[inline]
    pub fn new(n: u64) -> Self {
        debug_assert_eq!(n & 1, 1);
        let ni = (0..5).fold(n, |x, _a| {
            x.wrapping_mul(2u64.wrapping_sub(n.wrapping_mul(x)))
        });
        debug_assert_eq!(n.wrapping_mul(ni), 1);
        let nh = (n >> 1) + 1;
        let r = n.wrapping_neg() % n;
        let rn = n - r;
        let r2 = ((n as u128).wrapping_neg() % (n as u128)) as u64;
        let k = (n - 1).trailing_zeros();
        let d = (n - 1) >> k;

        Self {
            n,
            ni,
            nh,
            r,
            rn,
            r2,
            d,
            k,
        }
    }
    /// $ar(a) = a*r \mod N$
    #[inline]
    pub fn ar(&self, a: u64) -> u64 {
        debug_assert!(a < self.n);
        self.mrmul(a, self.r2)
    }
    /// $mrmul(ar, br) == (ar * br) / r \mod N$
    #[inline]
    pub fn mrmul(&self, ar: u64, br: u64) -> u64 {
        debug_assert!(ar < self.n);
        debug_assert!(br < self.n);
        let t: u128 = (ar as u128) * (br as u128);
        let (t, f) = ((t >> 64) as u64).overflowing_sub(
            ((((t as u64).wrapping_mul(self.ni) as u128) * self.n as u128) >> 64) as u64,
        );
        if f {
            t.wrapping_add(self.n)
        } else {
            t
        }
    }
    #[inline]
    pub fn pow(&self, mut ar: u64, mut b: u64) -> u64 {
        debug_assert!(ar < self.n);
        let mut t = if b & 1 == 0 { self.r } else { ar };
        b >>= 1;
        while b != 0 {
            ar = self.mrmul(ar, ar);
            if b & 1 != 0 {
                t = self.mrmul(t, ar);
            }
            b >>= 1;
        }
        t
    }
}

#[test]
fn test() {
    const MOD: u64 = 31;
    let mont = MontgomeryU64::new(MOD);
    for i in 0..MOD {
        assert_eq!(mont.ar(i), i * mont.r % MOD);
        for j in 0..MOD {
            assert_eq!(mont.mrmul(i, j) * mont.r % MOD, i * j % MOD);
        }
    }
}
