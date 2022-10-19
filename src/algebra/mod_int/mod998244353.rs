//! # $\mod 998244353$

use super::{Mod, ModInt};
use crate::prelude::*;

#[snippet(name = "mod998244353", doc_hidden)]
pub use mod_998_244_353_impl::{mi, Mi, Mod998_244_353};

#[snippet(name = "mod998244353", doc_hidden)]
pub mod mod_998_244_353_impl {
    use super::{Mod, ModInt, Pow, PrimitiveRoot, Zero};
    pub fn mi(i: u32) -> Mi {
        Mi::new(i)
    }

    pub type Mi = ModInt<Mod998_244_353>;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    pub struct Mod998_244_353;
    impl Mod for Mod998_244_353 {
        const MOD: u32 = 998_244_353;
        const MOD_INV: u32 = 3_296_722_945;
        const R: u32 = 301_989_884;
        const R_POW2: u32 = 932_051_910;
    }
    impl PrimitiveRoot for Mi {
        const DIVIDE_LIMIT: usize = 23;
        #[inline]
        fn primitive_root() -> Self {
            let exp = (Mi::zero() - 1) / Self::new(2).pow(23);
            Mi::pow(Self::new(3), exp.into())
        }
    }
}

#[test]
fn const_test() {
    use mod_998_244_353_impl::Mod998_244_353 as Mod;
    let n = Mod::MOD;
    let mut n_inv = n;
    // 5 times
    n_inv = n_inv.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(n_inv)));
    n_inv = n_inv.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(n_inv)));
    n_inv = n_inv.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(n_inv)));
    n_inv = n_inv.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(n_inv)));
    n_inv = n_inv.wrapping_mul(2u32.wrapping_sub(n.wrapping_mul(n_inv)));
    assert_eq!(n.wrapping_mul(n_inv), 1);
    assert_eq!(n_inv, Mod::MOD_INV);

    let r = n.wrapping_neg() % n;
    assert_eq!((1u64 << 32) % Mod::MOD as u64, r as u64);
    assert_eq!(r, Mod::R);

    let r2 = ((n as u64).wrapping_neg() % n as u64) as u32;
    assert_eq!(r2, Mod::R_POW2);
}

#[test]
fn a() {
    let exp = (Mi::zero() - 1) / Mi::new(2).pow(23);
    dbg!(exp);
    dbg!(Mi::new(3).pow(exp.into()));
    dbg!(Mi::new(998244352).pow(2));
}
