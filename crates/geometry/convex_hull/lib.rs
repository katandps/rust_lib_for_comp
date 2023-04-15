//! # 凸包
use min_max_macro::{chmax, max};
use plane_float::{ClockwiseDirection, Line, Point, Segment, EPS};
use prelude::*;

#[snippet(name = "convex-hull", doc_hidden)]
pub use convex_hull_impl::{Including, Polygon};
#[snippet(name = "convex-hull", doc_hidden)]
mod convex_hull_impl {
    use super::{chmax, max, ClockwiseDirection, Index, IndexMut, Line, Point, Segment, EPS};

    /// # 多角形
    #[derive(Clone, Debug)]
    pub struct Polygon {
        /// 頂点は反時計回りの順
        pub nodes: Vec<Point>,
    }

    impl From<&[(f64, f64)]> for Polygon {
        fn from(nodes: &[(f64, f64)]) -> Self {
            Self {
                nodes: nodes.iter().map(|&p| Point::from(p)).collect(),
            }
        }
    }

    /// # Indexアクセス
    /// 頂点番号 mod 頂点数でのアクセスを実装する
    impl Index<usize> for Polygon {
        type Output = Point;
        fn index(&self, index: usize) -> &Point {
            &self.nodes[(index) % self.nodes.len()]
        }
    }

    impl IndexMut<usize> for Polygon {
        fn index_mut(&mut self, index: usize) -> &mut Point {
            let n = self.nodes.len();
            &mut self.nodes[index % n]
        }
    }

    impl Polygon {
        pub fn new(nodes: Vec<Point>) -> Self {
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
        pub fn area(&self) -> f64 {
            let mut res = 0.0;
            for i in 0..self.nodes.len() {
                res += Point::cross(self[i], self[i + 1]);
            }
            res * 0.5
        }

        /// # 凸多角形の直径
        /// 最遠点対の距離をキャリパー法で求める
        ///
        /// ## 計算量
        /// $O(N\log N)$
        pub fn diameter(&self) -> f64 {
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
            let mut res = 0.0;
            let (si, sj) = (i, j);
            while i != sj || j != si {
                chmax!(res, Point::distance(&self[i], &self[j]));
                if Point::cross(self[i + 1] - self[i], self[j + 1] - self[j]) < 0.0 {
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
        pub fn include(&self, p: Point) -> Including {
            let mut sum = 0.0f64;
            for i in 0..self.nodes.len() {
                let (p1, p2) = (self.nodes[i], self.nodes[(i + 1) % self.nodes.len()]);
                if Segment::distance_to_point(Segment::new(p1, p2), p) < EPS {
                    return Including::OnLine;
                }
                let dot = Point::dot(p1 - p, p2 - p);
                let cross = Point::cross(p1 - p, p2 - p);
                sum += cross.atan2(dot)
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
        pub fn convex_hull(mut points: Vec<Point>, include_on_line: bool) -> Self {
            points.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mut nodes = Vec::new();
            for &p in &points {
                while nodes.len() >= 2 {
                    let c = Point::cross(
                        nodes[nodes.len() - 1] - nodes[nodes.len() - 2],
                        p - nodes[nodes.len() - 2],
                    );
                    if (include_on_line && c < -EPS) || (!include_on_line && c < EPS) {
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
                    let c = Point::cross(
                        nodes[nodes.len() - 1] - nodes[nodes.len() - 2],
                        p - nodes[nodes.len() - 1],
                    );
                    if (include_on_line && c < -EPS) || (!include_on_line && c < EPS) {
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
                let c1 = Point::cross(l.p2 - l.p1, self[i] - l.p1);
                let c2 = Point::cross(l.p2 - l.p1, self[i + 1] - l.p1);
                if c1 * c2 < EPS {
                    let edge = Line::new(self[i], self[i + 1]);
                    if let Some(cp) = Line::cross_point(edge, l) {
                        q.push(cp)
                    }
                }
                if c2 > -EPS {
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
