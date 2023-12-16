//! # 凸包
use super::plane_float::{ClockwiseDirection, Line, Segment, Vector};
use crate::element::float_value::FValue;
use crate::min_max_macro::{chmax, max};
use crate::prelude::*;

#[codesnip::entry("convex-hull")]
pub use convex_hull_impl::{Including, Polygon};
#[codesnip::entry(
    "convex-hull",
    include("chmax", "float-value", "plane-float", "prelude")
)]
mod convex_hull_impl {
    use super::{chmax, max, ClockwiseDirection, FValue, Index, IndexMut, Line, Segment, Vector};

    /// # 多角形
    #[derive(Clone, Debug)]
    pub struct Polygon {
        /// 頂点は反時計回りの順
        pub nodes: Vec<Vector>,
    }

    impl From<&[(f64, f64)]> for Polygon {
        fn from(nodes: &[(f64, f64)]) -> Self {
            Self {
                nodes: nodes.iter().map(|&p| Vector::from(p)).collect(),
            }
        }
    }

    /// # Indexアクセス
    /// 頂点番号 mod 頂点数でのアクセスを実装する
    impl Index<usize> for Polygon {
        type Output = Vector;
        fn index(&self, index: usize) -> &Vector {
            &self.nodes[(index) % self.nodes.len()]
        }
    }

    impl IndexMut<usize> for Polygon {
        fn index_mut(&mut self, index: usize) -> &mut Vector {
            let n = self.nodes.len();
            &mut self.nodes[index % n]
        }
    }

    impl Polygon {
        pub fn new(nodes: Vec<Vector>) -> Self {
            Self { nodes }
        }

        /// # 頂点数
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn number_of_sides(&self) -> usize {
            self.nodes.len()
        }

        /// # 面積
        /// 外積の総和/2
        ///
        /// ## 計算量
        /// $O(N)$
        pub fn area(&self) -> FValue {
            let mut res: FValue = 0.0.into();
            for i in 0..self.nodes.len() {
                res = res + Vector::cross(self[i], self[i + 1]);
            }
            res * 0.5
        }

        /// # 凸多角形の直径
        /// 最遠点対の距離をキャリパー法で求める
        ///
        /// ## 計算量
        /// $O(N\log N)$
        pub fn diameter(&self) -> FValue {
            assert!(self.is_convex());
            let n = self.nodes.len();
            let (mut i, mut j) = (0, 0);
            for k in 0..n {
                if self.nodes[k] < self.nodes[i] {
                    i = k
                }
                if self.nodes[j] < self.nodes[k] {
                    j = k
                }
            }
            let mut res = 0.0.into();
            let (si, sj) = (i, j);
            while i != sj || j != si {
                chmax!(res, Vector::distance(&self[i], &self[j]));
                if Vector::cross(self[i + 1] - self[i], self[j + 1] - self[j]) < 0.0.into() {
                    i = (i + 1) % n
                } else {
                    j = (j + 1) % n
                }
            }
            res
        }

        /// # 凸性判定
        ///
        /// ## 計算量
        /// $O(N)$
        pub fn is_convex(&self) -> bool {
            for i in 0..self.nodes.len() {
                if ClockwiseDirection::direction(self[i], self[i + 1], self[i + 2])
                    == ClockwiseDirection::Clockwise
                {
                    return false;
                }
            }
            true
        }

        /// # 内包判定(Winding number algorithm)
        /// 頂点の周りの角度を調べる 内包していないときは0に近い値になる
        ///
        /// ## 計算量
        /// $O(N)$
        ///
        /// ## todo
        /// atan2を使わない実装があるらしい(定数倍高速化)
        pub fn include(&self, p: Vector) -> Including {
            let mut sum = 0.0f64;
            for i in 0..self.nodes.len() {
                let (p1, p2) = (self.nodes[i], self.nodes[(i + 1) % self.nodes.len()]);
                if Segment::distance_to_point(Segment::new(p1, p2), p) < FValue::eps() {
                    return Including::OnLine;
                }
                let dot = Vector::dot(p1 - p, p2 - p);
                let cross = Vector::cross(p1 - p, p2 - p);
                sum += cross.0.atan2(dot.0)
            }
            // 角度和は$2\pi$の整数倍の値になる 誤差を考慮して$\pi$で判定する
            if sum.abs() >= std::f64::consts::PI {
                Including::Inside
            } else {
                Including::Outside
            }
        }

        /// # 凸包
        /// 隣り合う3点を見て、内側にはまっているものがないように頂点を削除する
        ///
        /// ## 引数
        /// - points: 頂点集合
        /// - include_on_line: trueならば辺上の点を含む
        ///
        /// ## 計算量
        /// $O(N\log N)$
        pub fn convex_hull(mut points: Vec<Vector>, include_on_line: bool) -> Self {
            points.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mut nodes = Vec::new();
            for &p in &points {
                while nodes.len() >= 2 {
                    let c = Vector::cross(
                        nodes[nodes.len() - 1] - nodes[nodes.len() - 2],
                        p - nodes[nodes.len() - 2],
                    );
                    if (include_on_line && c < (-FValue::eps()))
                        || (!include_on_line && c < FValue::eps())
                    {
                        nodes.pop();
                    } else {
                        break;
                    }
                }
                nodes.push(p);
            }
            let m = nodes.len();
            points.reverse();
            for &p in &points {
                while nodes.len() > m {
                    let c = Vector::cross(
                        nodes[nodes.len() - 1] - nodes[nodes.len() - 2],
                        p - nodes[nodes.len() - 1],
                    );
                    if (include_on_line && c < (-FValue::eps()))
                        || (!include_on_line && c < FValue::eps())
                    {
                        nodes.pop();
                    } else {
                        break;
                    }
                }
                // 同じ頂点は含まないようにする
                if nodes[nodes.len() - 1] != p {
                    nodes.push(p);
                }
            }
            // 同じ頂点は含まないようにする
            if nodes[0] == nodes[nodes.len() - 1] {
                nodes.pop();
            }

            Self::new(nodes)
        }

        /// # $x$について正規化
        /// 最も左にある点のうち、最も下にあるものが頂点0になるよう、回転させる
        ///
        /// ## 計算量
        /// $O(N)$
        pub fn normalize(&mut self) {
            let mut result = Vec::new();
            let mut start = false;
            for i in 0..self.nodes.len() * 2 {
                if result.len() == self.nodes.len() {
                    break;
                }
                let cur = i;
                if start {
                    result.push(self[cur]);
                } else {
                    let prev = i + self.nodes.len() - 1;
                    let next = i + 1;
                    if self[prev] > self[cur] && self[cur] < self[next] {
                        start = true;
                        result.push(self[cur]);
                    }
                }
            }
            self.nodes = result
        }

        /// # 凸多角形の切断
        ///
        pub fn cut(&self, l: Line) -> Self {
            let mut q = Vec::new();
            for i in 0..self.nodes.len() {
                let c1 = Vector::cross(l.p2 - l.p1, self[i] - l.p1);
                let c2 = Vector::cross(l.p2 - l.p1, self[i + 1] - l.p1);
                if c1 * c2 < FValue::eps() {
                    let edge = Line::new(self[i], self[i + 1]);
                    if let Some(cp) = Line::cross_point(edge, l) {
                        q.push(cp)
                    }
                }
                if c2 > -FValue::eps() {
                    q.push(self[i + 1])
                }
            }
            Self::new(q)
        }
    }

    pub enum Including {
        Inside,
        OnLine,
        Outside,
    }
}
