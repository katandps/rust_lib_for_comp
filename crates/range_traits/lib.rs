use algebra::*;
use prelude::*;

#[snippet(name = "range-traits", doc_hidden)]
#[rustfmt::skip]
pub trait ToBounds<T> {
    fn lr(&self) -> (T, T);
}

#[snippet(name = "range-traits", doc_hidden)]
#[rustfmt::skip]
impl<R: RangeBounds<T> + Clone, T: Copy + BoundedAbove + BoundedBelow + One + Add<Output = T>> ToBounds<T>
    for R
{
    #[inline]
    fn lr(&self) -> (T, T) {
        use Bound::{Excluded, Included, Unbounded};
        let l = match self.start_bound() {
            Unbounded => T::min_value(),
            Included(&s) => s,
            Excluded(&s) => s + T::one(),
        };
        let r = match self.end_bound() {
            Unbounded => T::max_value(),
            Included(&e) => e + T::one(),
            Excluded(&e) => e,
        };
        (l, r)
    }
}

/// # 二項演算の総積クエリを提供する
/// 遅延評価などを持つデータ構造は、&mut selfを要求するRangeProductMutを使用する
#[snippet(name = "range-traits", doc_hidden)]
#[rustfmt::skip]
pub trait RangeProduct<I> {
    type Magma: Magma;
    fn product<R: ToBounds<I>>(&self, range: R) -> <Self::Magma as Magma>::M;
}

#[snippet(name = "range-traits", doc_hidden)]
#[rustfmt::skip]
pub trait RangeProductMut<I> {
    type Magma: Magma;
    fn product<R: ToBounds<I>>(&mut self, range: R) -> <Self::Magma as Magma>::M;
}
