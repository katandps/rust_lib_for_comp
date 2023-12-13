//! # $\mod 2^{64}$ における乗法逆元
//!
//! ## 計算量
//! $O(\log N)$

use prelude::*;

pub trait ModInvU64 {
    fn inv(self) -> Self;
}

impl ModInvU64 for u64 {
    fn inv(self) -> Self {
        assert_eq!(self & 1, 1); // 奇数でないとダメ
        let mut ni = self;
        for _ in 0..5 {
            ni = ni.wrapping_mul(2u64.wrapping_sub(self.wrapping_mul(ni)))
        }
        assert_eq!(self.wrapping_mul(ni), 1);
        ni
    }
}

#[test]
fn test2() {
    for i in 1..1000000 {
        let n: u64 = i * 2 + 1;
        let inv = n.inv();
        assert_eq!((n as i128 * inv as i128) % (1i128 << 64), 1);
    }
}

pub trait ModInv {
    fn inv(self, modulo: Self) -> Self;
}

impl ModInv for i64 {
    fn inv(mut self, modulo: Self) -> Self {
        let (mut b, mut u, mut v) = (modulo, 1, 0);
        while b > 0 {
            let t = self / b;
            self -= t * b;
            swap(&mut self, &mut b);
            u -= t * v;
            swap(&mut u, &mut v);
        }
        u %= modulo;
        if u < 0 {
            u += modulo;
        }
        u
    }
}

#[test]
fn test() {
    for i in 1i64..13 {
        let inv = i.inv(13);
        assert_eq!(i * inv % 13, 1);
    }
}
