//! # 半開区間への変換
use crate::prelude::*;

#[snippet(name = "range", doc_hidden)]
#[rustfmt::skip]
pub trait ToLR<T> {
    fn to_lr(&self) -> (T, T);
}

#[snippet(name = "range", doc_hidden)]
#[rustfmt::skip]
impl<R: RangeBounds<T>, T: Copy + BoundedAbove + BoundedBelow + One + Add<Output = T>> ToLR<T>
    for R
{
    fn to_lr(&self) -> (T, T) {
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
