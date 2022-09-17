//! #ModInt(mod 998244353)

use super::{Mod, ModInt};
use crate::prelude::*;

#[snippet(name = "mod998244353", doc_hidden)]
pub fn mi(i: i64) -> Mi {
    Mi::new(i)
}

#[snippet(name = "mod998244353", doc_hidden)]
pub type Mi = ModInt<Mod998244353>;

#[snippet(name = "mod998244353", doc_hidden)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Mod998244353;
#[snippet(name = "mod998244353", doc_hidden)]
impl Mod for Mod998244353 {
    fn get() -> i64 {
        998_244_353
    }
}
