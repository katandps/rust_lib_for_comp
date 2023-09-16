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
    }
}

#[test]
fn const_test() {
    use mod_1_000_000_007_impl::Mod1_000_000_007 as Mod;
    assert_eq!(Mod::MOD.wrapping_mul(Mod::MOD_INV), 1);
    assert_eq!(((1u64 << 32) % Mod::MOD as u64) as u32, Mod::R);
    assert_eq!(
        ((Mod::MOD as u64).wrapping_neg() % Mod::MOD as u64) as u32,
        Mod::R_POW2
    );
}
