//! # 二分探索
//!
//! 整数での二分探索を行う
//!
//! ## 使い方
//!
//! ```
//! # use rust_lib_for_comp::algo::binary_search::*;
//! let src = vec![1, 5, 15, 30, 55, 90, 150];
//! let res = binary_search(src.len() as i64, -1, |i| src[i as usize] > 55);
//! assert_eq!(src[res as usize], 90);
//! ```

use crate::prelude::*;

#[snippet(name = "binary-search", doc_hidden)]
pub fn binary_search<F: Fn(i64) -> bool>(mut ok: i64, mut ng: i64, f: F) -> i64 {
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid
        } else {
            ng = mid
        }
    }
    ok
}
