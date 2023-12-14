//! # index付きの値
//! Max, Minのみ実装済み
use algebra::Integral;
use prelude::*;

#[snippet(name = "int-with-index", doc_hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntWithIndex<V: Integral, I: Integral> {
    pub value: V,
    pub index: I,
}
#[snippet(name = "int-with-index", doc_hidden)]
impl<V: Integral, I: Integral> PartialOrd for IntWithIndex<V, I> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[snippet(name = "int-with-index", doc_hidden)]
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

#[snippet(name = "int-with-index", doc_hidden)]
impl<V: Integral, I: Integral> From<(I, V)> for IntWithIndex<V, I> {
    fn from((index, value): (I, V)) -> Self {
        IntWithIndex { value, index }
    }
}
