//! # 一括import用
//!

#[codesnip::entry("prelude")]
pub use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{hash_map::RandomState, BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    convert::Infallible,
    convert::{TryFrom, TryInto},
    default::Default,
    fmt::{Debug, Display, Formatter},
    hash::{BuildHasherDefault, Hash, Hasher},
    io::{stdin, stdout, BufRead, BufWriter, Read, StdoutLock, Write},
    iter::{repeat, FromIterator, Product, Sum},
    marker::PhantomData,
    mem::swap,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Bound,
        Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Range,
        RangeBounds, RangeInclusive, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
    str::{from_utf8, FromStr},
};

/// # 各種Trait
pub mod traits {
    pub use crate::algebra::*;
    pub use crate::algo::slice_bounds::*;
    pub use crate::graph::GraphTrait;
    pub use crate::range_traits::*;
    pub use crate::util::string_util::*;
}
