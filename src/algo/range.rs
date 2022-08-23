//! Range を半開区間 $[l, r)$ に変換する
use crate::prelude::*;

#[snippet(name = "range", doc_hidden)]
pub fn to_lr<T, R: RangeBounds<T>>(range: &R, length: T) -> (T, T)
where
    T: Copy + One + Zero + Add<Output = T> + PartialOrd,
{
    use Bound::{Excluded, Included, Unbounded};
    let l = match range.start_bound() {
        Unbounded => T::zero(),
        Included(&s) => s,
        Excluded(&s) => s + T::one(),
    };
    let r = match range.end_bound() {
        Unbounded => length,
        Included(&e) => e + T::one(),
        Excluded(&e) => e,
    };
    assert!(l <= r && r <= length);
    (l, r)
}
