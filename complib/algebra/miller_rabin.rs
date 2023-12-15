//! # Miller-Rabin素数判定法
//! ここでは、$2^{64} - 1$ 以下の数について、決定的アルゴリズムとして扱う
//!
//! ## dependency
//! [モンゴメリ乗算](montgomery_multiplication_64)
//!
use crate::algebra::montgomery_multiplication_64::MontgomeryReduction;

#[codesnip::entry("miller-rabin", include("montgomery-multiplication"))]
pub trait MillerRabin {
    /// 素数判定
    fn is_prime(&self) -> bool;
}

#[codesnip::entry("miller-rabin", include("montgomery-multiplication"))]
impl MillerRabin for u64 {
    fn is_prime(&self) -> bool {
        if *self < 2 || *self & 1 == 0 {
            return *self == 2; // 偶数は2だけ素数
        }
        let mont = MontgomeryReduction::new(*self);
        let is_composite = |mut checker: u64| -> bool {
            if checker >= *self {
                checker %= self;
            }
            if checker == 0 {
                return false;
            }
            let mut tr = mont.pow(checker, mont.d);
            if tr == mont.r || tr == mont.r_neg {
                return false;
            }
            (1..mont.k).all(|_| {
                tr = mont.mrmul(tr, tr);
                tr != mont.r_neg
            })
        };
        const MILLER_RABIN_BASES_32: [u64; 3] = [2, 7, 61];
        const MILLER_RABIN_BASES_64: [u64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
        if *self < 1 << 32 {
            MILLER_RABIN_BASES_32.iter()
        } else {
            MILLER_RABIN_BASES_64.iter()
        }
        .all(|&checker| !is_composite(checker)) // すべてのcheckerについてすべて合成数と判定されなかった <=> selfが素数
    }
}

#[test]
fn test() {
    assert_eq!(false, 0.is_prime());
    assert_eq!(false, 1.is_prime());
    assert_eq!(true, 2.is_prime());
    assert_eq!(true, 3.is_prime());
    assert_eq!(false, 4.is_prime());
    assert_eq!(true, 5.is_prime());
    assert_eq!(false, 99.is_prime());
    assert_eq!(false, 100.is_prime());
    assert_eq!(true, 101.is_prime());
    assert_eq!(false, 1565912117761.is_prime());
}
