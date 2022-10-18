//! # $\mod 998244353$

use super::{Mod, ModInt};
use crate::{algebra::montgomery_multiplication::MontgomeryReduction, prelude::*};

#[snippet(name = "mod998244353", doc_hidden)]
pub use mod_998_244_353_impl::{mi, Mi};

#[snippet(name = "mod998244353", doc_hidden)]
pub mod mod_998_244_353_impl {
    use super::{Mod, ModInt, MontgomeryReduction, Pow, PrimitiveRoot, Zero};
    pub fn mi(i: i64) -> Mi {
        Mi::new(i)
    }

    pub type Mi = ModInt<Mod998_244_353>;

    const MOD: i64 = 998_244_353;
    // const MONTGOMERY: MontgomeryU64 = MontgomeryU64::new(MOD as u64);

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
    pub struct Mod998_244_353;
    impl Mod for Mod998_244_353 {
        fn get() -> i64 {
            MOD as i64
        }
        fn mont() -> MontgomeryReduction {
            MontgomeryReduction::new(MOD as u64)
        }
    }
    impl PrimitiveRoot for Mi {
        fn primitive_root() -> Self {
            let exp = (Mi::zero() - 1) / Self::new(2).pow(23);
            Mi::pow(Self::new(3), exp.get())
        }
    }
}
