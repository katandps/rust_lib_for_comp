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
