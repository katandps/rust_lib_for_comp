//! Range を半開区間 $`[l, r)`$ に変換する
use crate::prelude::*;

pub fn to_lr<R: RangeBounds<usize>>(range: &R, length: usize) -> (usize, usize) {
    let l = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&s) => s,
        Bound::Excluded(&s) => s + 1,
    };
    let r = match range.end_bound() {
        Bound::Unbounded => length,
        Bound::Included(&e) => e + 1,
        Bound::Excluded(&e) => e,
    };
    assert!(l <= r && r <= length);
    (l, r)
}
