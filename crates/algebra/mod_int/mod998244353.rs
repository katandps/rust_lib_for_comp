//! # $\mod 998244353$

use super::{Mod, ModInt};
use algebra::*;
use prelude::*;

#[snippet(name = "mod998244353", doc_hidden)]
pub use mod_998_244_353_impl::{Mi, Mod998_244_353};

#[snippet(name = "mod998244353", doc_hidden)]
pub mod mod_998_244_353_impl {
    use super::{Mod, ModInt, PrimitiveRoot};

    pub type Mi = ModInt<Mod998_244_353>;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    pub struct Mod998_244_353;
    impl Mod for Mod998_244_353 {
        const MOD: u32 = 998_244_353;
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
    assert_eq!(Mod::MOD.wrapping_mul(Mod::MOD_INV), 1);
    assert_eq!(((1u64 << 32) % Mod::MOD as u64) as u32, Mod::R);
    assert_eq!(
        ((Mod::MOD as u64).wrapping_neg() % Mod::MOD as u64) as u32,
        Mod::R_POW2
    );
}

#[test]
fn primitive_root() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for i in 0..Mi::DIVIDE_LIMIT {
        let n = Mi::primitive_root().pow((1 << i).into());
        set.insert(n);
        assert_eq!(n.pow(1 << (23 - i)), Mi::one());
    }
    assert_eq!(set.len(), 23);
}
