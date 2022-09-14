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
