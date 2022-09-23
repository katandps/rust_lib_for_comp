//! Range を半開区間 $[l, r)$ に変換する
use crate::prelude::*;

// pub fn to_lr<T, R: RangeBounds<T>>(range: &R, length: T) -> (T, T)
// where
//     T: Copy + One + Zero + Add<Output = T> + PartialOrd,
// {
//     use Bound::{Excluded, Included, Unbounded};
//     let l = match range.start_bound() {
//         Unbounded => T::zero(),
//         Included(&s) => s,
//         Excluded(&s) => s + T::one(),
//     };
//     let r = match range.end_bound() {
//         Unbounded => length,
//         Included(&e) => e + T::one(),
//         Excluded(&e) => e,
//     };
//     assert!(l <= r && r <= length);
//     (l, r)
// }

#[snippet(name = "range", doc_hidden)]
#[rustfmt::skip]
pub trait ToLR<T> {
    fn to_lr(&self) -> (T, T);
}
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
