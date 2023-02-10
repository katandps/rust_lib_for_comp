//! # $\mod 2^{64}$ における乗法逆元

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
fn test() {
    for i in 1..1000000 {
        let n = i * 2 + 1;
        let inv = n.inv();
        assert_eq!((n as i128 * inv as i128) % (1i128 << 64), 1);
    }
}

//! # 剰余体における逆元
//!
//! ## 計算量
//! $O(\log N)$

use crate::prelude::*;

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
    assert_eq!(1, 1.inv(13));
    assert_eq!(7, 2.inv(13));
    assert_eq!(9, 3.inv(13));
    assert_eq!(10, 4.inv(13));
    assert_eq!(8, 5.inv(13));
    assert_eq!(11, 6.inv(13));
    assert_eq!(2, 7.inv(13));
    assert_eq!(5, 8.inv(13));
    assert_eq!(3, 9.inv(13));
    assert_eq!(4, 10.inv(13));
    assert_eq!(6, 11.inv(13));
    assert_eq!(12, 12.inv(13));
    assert_eq!(0, 13.inv(13));
    assert_eq!(1, 14.inv(13));
}
