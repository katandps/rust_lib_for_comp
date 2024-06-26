use crate::algebra::*;
use crate::prelude::*;

#[codesnip::entry("range-traits", include("algebra", "prelude"))]
pub use range_traits_impl::{PointUpdate, RangeProduct, RangeProductMut, RangeUpdate, ToBounds};

#[codesnip::entry("range-traits", include("algebra", "prelude"))]
mod range_traits_impl {
    use super::{Add, Bound, BoundedAbove, BoundedBelow, Magma, One, RangeBounds};
    pub trait ToBounds<T> {
        fn lr(&self) -> (T, T);
    }
    impl<
            R: RangeBounds<T> + Clone,
            T: Copy + BoundedAbove + BoundedBelow + One + Add<Output = T>,
        > ToBounds<T> for R
    {
        #[inline]
        fn lr(&self) -> (T, T) {
            let l = match self.start_bound() {
                Bound::Unbounded => T::min_value(),
                Bound::Included(&s) => s,
                Bound::Excluded(&s) => s + T::one(),
            };
            let r = match self.end_bound() {
                Bound::Unbounded => T::max_value(),
                Bound::Included(&e) => e + T::one(),
                Bound::Excluded(&e) => e,
            };
            (l, r)
        }
    }

    /// # 二項演算の総積クエリを提供する
    /// 遅延評価などを持つデータ構造は、&mut selfを要求するRangeProductMutを使用する
    pub trait RangeProduct<I> {
        type Magma: Magma;
        fn product<R: ToBounds<I>>(&self, range: R) -> <Self::Magma as Magma>::M;
    }

    pub trait RangeProductMut<I> {
        type Magma: Magma;
        fn product<R: ToBounds<I>>(&mut self, range: R) -> <Self::Magma as Magma>::M;
    }

    impl<T: RangeProduct<I>, I> RangeProductMut<I> for T {
        type Magma = T::Magma;
        fn product<R: ToBounds<I>>(&mut self, range: R) -> <Self::Magma as Magma>::M {
            <Self as RangeProduct<I>>::product(self, range)
        }
    }

    /// # 値の更新
    /// indexで指定した値をfで更新する
    pub trait PointUpdate<T> {
        fn update_at(&mut self, index: usize, f: T);
    }

    /// # 区間の更新
    /// rangeで指定した値をfで更新する
    pub trait RangeUpdate<I, T> {
        fn update_range<R: ToBounds<I>>(&mut self, range: R, f: T);
    }
}
