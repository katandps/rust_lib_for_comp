//! # 一括インポート用モジュール
//! ここでインポートする要素はtemplateで使えるようにしておく

#[snippet(name = "prelude", doc_hidden)]
#[rustfmt::skip]
pub use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{
        hash_map::RandomState, BTreeMap, BTreeSet, BinaryHeap, VecDeque,
    },
    convert::Infallible,
    convert::{TryFrom, TryInto},
    fmt::{Debug, Display, Formatter},
    hash::{Hash, BuildHasherDefault, Hasher},
    io::{stdin, stdout, BufRead, BufWriter, Read, Write},
    iter::{repeat, Product, Sum},
    marker::PhantomData,
    mem::swap,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Bound,
        Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Range,
        RangeBounds, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    str::{from_utf8, FromStr},
};

pub use crate::algebra::*;
pub use crate::algo::fxhasher::*;
pub use crate::algo::range::ToLR;
pub use crate::util::{reader::*, writer::*};
pub use cargo_snippet::snippet;
