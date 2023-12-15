//! # index付きの値
//! Max, Minのみ実装済み
use crate::algebra::Integral;
use crate::prelude::*;

#[codesnip::entry("int-with-index")]
pub use int_with_index_impl::IntWithIndex;
#[codesnip::entry("int-with-index", include("algebra", "prelude"))]
mod int_with_index_impl {
    use super::{Integral, Ordering};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct IntWithIndex<V: Integral, I: Integral> {
        pub value: V,
        pub index: I,
    }
    #[codesnip::entry("int-with-index", doc_hidden)]
    impl<V: Integral, I: Integral> PartialOrd for IntWithIndex<V, I> {
        fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
            Some(self.cmp(rhs))
        }
    }

    #[codesnip::entry("int-with-index", doc_hidden)]
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

    #[codesnip::entry("int-with-index", doc_hidden)]
    impl<V: Integral, I: Integral> From<(I, V)> for IntWithIndex<V, I> {
        fn from((index, value): (I, V)) -> Self {
            IntWithIndex { value, index }
        }
    }
}
