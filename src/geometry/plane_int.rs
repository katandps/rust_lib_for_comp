//! # 二次元平面(整数範囲)
use crate::prelude::*;

#[snippet(name = "plane-int", doc_hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    x: i64,
    y: i64,
}

#[snippet(name = "plane-int", doc_hidden)]
impl Point {
    /// ## 原点を軸にpi/2回転させる
    pub fn rot90(self) -> Point {
        Self::from((-self.y, self.x))
    }

    /// ## x軸に対して反転
    pub fn conj(self) -> Point {
        Self::from((self.x, -self.y))
    }

    /// ## 外積
    pub fn cross(&self, q: &Self) -> i64 {
        self.x * q.y - self.y * q.x
    }

    /// ## 内積
    pub fn dot(&self, q: &Self) -> i64 {
        self.x * q.x + self.y * q.y
    }

    /// ## ノルム
    pub fn norm(&self) -> i64 {
        self.dot(self)
    }

    /// # 偏角ソートに使える比較関数
    /// 偏角で比較する $`0 <= \theta < 2\pi`$ における偏角の大小 ソート関数に使用できる
    ///
    /// ## 使い方
    /// (0, 0)をうまく比較できないので先に除いておくこと
    ///
    /// ```
    /// # use itertools::Itertools;
    /// # use rust_lib_for_comp::geometry::plane_int::*;
    /// let pts = [
    ///     (1, 0),
    ///     (1, 1),
    ///     (0, 1),
    ///     (-1, 1),
    ///     (-1, 0),
    ///     (-1, -1),
    ///     (0, -1),
    ///     (1, -1),
    /// ];
    /// for (i, j) in (0..8).cartesian_product(0..8) {
    ///     assert_eq!(Point::arg_cmp(&pts[i].into(), &pts[j].into()), i.cmp(&j));
    /// }
    /// ```
    pub fn arg_cmp(p1: &Self, p2: &Self) -> Ordering {
        ((p1.y, p1.x) < (0, 0))
            .cmp(&((p2.y, p2.x) < (0, 0)))
            .then_with(|| (p2.x * p1.y).cmp(&(p1.x * p2.y)))
    }
}
#[snippet(name = "plane-int", doc_hidden)]

impl From<(i64, i64)> for Point {
    fn from(p: (i64, i64)) -> Point {
        Point { x: p.0, y: p.1 }
    }
}
