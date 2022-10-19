//! # $\mod 1000000007$
use super::{Mod, ModInt};
use crate::algebra::montgomery_multiplication::MontgomeryReduction;
use crate::prelude::*;

#[snippet(name = "mod1000000007", doc_hidden)]
pub use mod_1_000_000_007_impl::{mi, Mi, Mod1_000_000_007};

#[snippet(name = "mod1000000007", doc_hidden)]
mod mod_1_000_000_007_impl {
    use super::{Mod, ModInt, MontgomeryReduction};
    pub fn mi(i: i64) -> Mi {
        Mi::new(i)
    }

    pub type Mi = ModInt<Mod1_000_000_007>;

    const MOD: i64 = 1_000_000_007;
    const MONTGOMERY: MontgomeryReduction = MontgomeryReduction::new(MOD as u64);

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    pub struct Mod1_000_000_007;
    impl Mod for Mod1_000_000_007 {
        fn get() -> i64 {
            MOD
        }
        fn mont() -> MontgomeryReduction {
            MONTGOMERY
        }
    }
}
