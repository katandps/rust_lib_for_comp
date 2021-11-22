//! # 区間で与えられた集合の和を取る
//!
//! ## 使い方
//! ```
//! # use rust_lib_for_comp::algo::union_of_ranges::*;
//! let ranges = vec![-2..5, 3..7, 7..10, 14..15, 11..18, -100000..-2345];
//! assert_eq!(3, unite(&ranges).len());
//! let ranges = vec![-2..=5, -2..=13, 2..=17, 10..=17];
//! assert_eq!(vec![-2..18], unite(&ranges));
//! ```
//!
//! ## verify
//! [ABC147F](https://atcoder.jp/contests/abc147/submissions/27444751)
use crate::prelude::*;

#[snippet(name = "union-of-ranges", doc_hidden)]
pub fn unite<R: RangeBounds<i64>>(set: &[R]) -> Vec<Range<i64>> {
    let mut b: Vec<_> = set
        .iter()
        .map(|r| {
            use Bound::{Excluded, Included, Unbounded};
            let l = match r.start_bound() {
                Unbounded => std::i64::MIN,
                Included(&s) => s,
                Excluded(&s) => s + 1,
            };
            let r = match r.end_bound() {
                Unbounded => std::i64::MAX,
                Included(&e) => e + 1,
                Excluded(&e) => e,
            };
            (l, r)
        })
        .collect();
    b.sort();
    let mut iter = b.into_iter();
    if let Some((l1, r1)) = iter.next() {
        let mut v = vec![(l1, r1)];
        iter.for_each(|(li, ri)| {
            let last = v.len() - 1;
            if v[last].1 >= li {
                v[last] = (v[last].0, max(v[last].1, ri));
            } else {
                v.push((li, ri));
            }
        });
        v.into_iter().map(|(l, r)| l..r).collect()
    } else {
        Vec::new()
    }
}
