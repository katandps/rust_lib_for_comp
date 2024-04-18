//! # index付きの値
//! Max, Minのみ実装済み
use crate::algebra::Integral;
use crate::prelude::*;

#[codesnip::entry("int-with-index")]
pub use int_with_index_impl::IntWithIndex;
#[codesnip::entry("int-with-index", include("algebra", "prelude"))]
mod int_with_index_impl {
    use super::{Display, Formatter, Integral, Ordering};

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
    pub struct IntWithIndex<V: Integral, I: Integral> {
        pub value: V,
        pub index: I,
    }
    impl<V: Integral, I: Integral> PartialOrd for IntWithIndex<V, I> {
        fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
            Some(self.cmp(rhs))
        }
    }

    impl<V: Integral, I: Integral> Ord for IntWithIndex<V, I> {
        fn cmp(&self, rhs: &Self) -> Ordering {
            use Ordering::*;
            match self.value.cmp(&rhs.value) {
                Greater => Greater,
                Less => Less,
                Equal => self.index.cmp(&rhs.index),
            }
        }
    }

    impl<V: Integral, I: Integral> From<(I, V)> for IntWithIndex<V, I> {
        fn from((index, value): (I, V)) -> Self {
            IntWithIndex { value, index }
        }
    }

    impl<V: Integral, I: Integral> Display for IntWithIndex<V, I> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "{}:{}", self.index, self.value)
        }
    }
}
