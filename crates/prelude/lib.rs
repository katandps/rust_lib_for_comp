pub use cargo_snippet::snippet;

#[snippet(name = "prelude", doc_hidden)]
#[rustfmt::skip]
pub use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{
        hash_map::RandomState, BTreeMap, BTreeSet, BinaryHeap, VecDeque,
    },
    convert::Infallible,
    convert::{TryFrom, TryInto},
    default::Default,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, BuildHasherDefault, Hasher},
    io::{stdin, stdout, BufRead, BufWriter, Read, Write,StdoutLock},
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
