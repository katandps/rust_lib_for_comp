//! # 偏角の比較
//! 偏角ソートに使える比較関数
//!
//! //! ## 使い方
//! (0, 0)をうまく比較できないので先に除いておくこと
//! ```
//! # use itertools::Itertools;
//! # use rust_lib_for_comp::geometry::arg_cmp::*;
//! let pts = [
//!     (1, 0),
//!     (1, 1),
//!     (0, 1),
//!     (-1, 1),
//!     (-1, 0),
//!     (-1, -1),
//!     (0, -1),
//!     (1, -1),
//! ];
//! for (i, j) in (0..8).cartesian_product(0..8) {
//!     assert_eq!(arg_cmp(&pts[i].into(), &pts[j].into()), i.cmp(&j));
//! }
//! ```
//!
use crate::prelude::*;

/// # 整数範囲の点
#[snippet(name = "arg-cmp", doc_hidden)]
#[derive(Clone, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

/// (x, y)から点の構造体を生成する
#[snippet(name = "arg-cmp", doc_hidden)]
impl From<(i64, i64)> for Point {
    fn from(p: (i64, i64)) -> Point {
        Point { x: p.0, y: p.1 }
    }
}

/// # 整数範囲で偏角のcmp関数
#[snippet(name = "arg-cmp", doc_hidden)]
pub fn arg_cmp(p1: &Point, p2: &Point) -> Ordering {
    ((p1.y, p1.x) < (0, 0))
        .cmp(&((p2.y, p2.x) < (0, 0)))
        .then_with(|| (p2.x * p1.y).cmp(&(p1.x * p2.y)))
}
