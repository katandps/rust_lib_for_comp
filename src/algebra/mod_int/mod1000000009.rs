//! #ModInt(mod 1000000009)

use super::{Mod, ModInt};
use crate::prelude::*;

#[snippet(name = "mod1000000009", doc_hidden)]
pub fn mi(i: i64) -> Mi {
    Mi::new(i)
}

#[snippet(name = "mod1000000009", doc_hidden)]
pub type Mi = ModInt<Mod1000000009>;

#[snippet(name = "mod1000000009", doc_hidden)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Mod1000000009;
#[snippet(name = "mod1000000009", doc_hidden)]
impl Mod for Mod1000000009 {
    fn get() -> i64 {
        1_000_000_009
    }
}
