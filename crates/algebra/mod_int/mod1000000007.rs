//! # $\mod 1000000007$
use super::{Mod, ModInt};
use prelude::*;

#[snippet(name = "mod1000000007", doc_hidden)]
pub use mod_1_000_000_007_impl::{mi, Mi, Mod1_000_000_007};

#[snippet(name = "mod1000000007", doc_hidden)]
mod mod_1_000_000_007_impl {
    use super::{Mod, ModInt};
    pub fn mi(i: i64) -> Mi {
        Mi::from(i)
    }

    pub type Mi = ModInt<Mod1_000_000_007>;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    pub struct Mod1_000_000_007;
    impl Mod for Mod1_000_000_007 {
        const MOD: u32 = 1_000_000_007;
        const MOD_INV: u32 = 2_068_349_879;
        const R: u32 = 294_967_268;
        const R_POW2: u32 = 582_344_008;
    }
}

#[test]
fn const_test() {
    use mod_1_000_000_007_impl::Mod1_000_000_007 as Mod;
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
