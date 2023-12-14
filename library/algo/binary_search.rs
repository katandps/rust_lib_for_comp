//! # 二分探索
//!
//! 二分探索のテンプレート

use crate::prelude::*;

/// # 整数範囲の二分探索
///
/// ## 使い方
/// ```
/// # use binary_search::*;
/// let src = vec![1, 5, 15, 30, 55, 90, 150];
/// let res = binary_search(src.len() as i64, -1, |i| src[i as usize] > 55);
/// assert_eq!(src[res as usize], 90);
/// ```
#[codesnip::entry("binary-search", include("prelude"))]
pub fn binary_search<F: Fn(i64) -> bool>(mut ok: i64, mut ng: i64, f: F) -> i64 {
    while (ok - ng).abs() > 1 {
        let mut mid = (ok + ng) / 2;
        let r = f(mid);
        swap(&mut mid, if r { &mut ok } else { &mut ng });
    }
    ok
}

/// # 浮動小数点数の二分探索
///
/// ## 使い方
/// ```
/// # use binary_search::*;
/// let res = binary_search_float(1e9, 0.0, |m| m >= 5.0);
/// assert!((5.0-res).abs() < 1e-12);
/// ```
#[codesnip::entry("binary-search-float", doc_hidden)]
pub fn binary_search_float<F: Fn(f64) -> bool>(mut ok: f64, mut ng: f64, f: F) -> f64 {
    const EPS: f64 = 1e-15;
    while (ok - ng).abs() > EPS {
        let mut mid = (ok + ng) / 2.0;
        let r = f(mid);
        swap(&mut mid, if r { &mut ok } else { &mut ng });
    }
    ok
}
