//! # index付きの値
//! Max, Minのみ実装済み
use algebra::Integral;
use prelude::*;

#[snippet(name = "int-with-index", doc_hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntWithIndex<I: Integral> {
    pub value: I,
    pub index: usize,
}
#[snippet(name = "int-with-index", doc_hidden)]
impl<I: Integral> PartialOrd for IntWithIndex<I> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[snippet(name = "int-with-index", doc_hidden)]
impl<I: Integral> Ord for IntWithIndex<I> {
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
impl<I: Integral> From<(usize, I)> for IntWithIndex<I> {
    fn from((index, value): (usize, I)) -> Self {
        IntWithIndex { value, index }
    }
}
