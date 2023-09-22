//! # 凸包
use float_value::FValue;
use min_max_macro::{chmax, max};
use plane_float::{ClockwiseDirection, Line, Segment, Vector};
use prelude::*;

#[snippet(name = "convex-hull", doc_hidden)]
pub use convex_hull_impl::{Including, Polygon};
#[snippet(name = "convex-hull", doc_hidden)]
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
                    if (include_on_line && c < -FValue::eps())
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
                    if (include_on_line && c < -FValue::eps())
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

/// <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_3_A>
pub fn cgl_3_a(_n: usize, xy: Vec<(f64, f64)>) -> float_value::FValue {
    let polygon = Polygon::from(&xy[..]);
    polygon.area()
}

#[test]
fn cgl_3_a_test() {
    let n = 4;
    let xy = vec![(0.0, 0.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    assert_eq!(1.5, cgl_3_a(n, xy).0)
}

/// <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/4/CGL_4_C>
pub fn cgl_4_c(
    _n: usize,
    xy: Vec<(f64, f64)>,
    _q: usize,
    p: Vec<(f64, f64, f64, f64)>,
) -> Vec<FValue> {
    let points = xy
        .into_iter()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let polygon = Polygon::convex_hull(points, true);
    let mut ret = Vec::new();
    for (p1x, p1y, p2x, p2y) in p {
        let line = Line::new(Vector::new(p1x, p1y), Vector::new(p2x, p2y));
        let ans = polygon.cut(line);
        assert!(ans.is_convex());
        ret.push(ans.area());
    }
    ret
}

#[test]
fn cgl_4_c_test() {
    let n = 4;
    let xy = vec![(1.0, 1.0), (4.0, 1.0), (4.0, 3.0), (1.0, 3.0)];
    let q = 2;
    let p = vec![(2.0, 0.0, 2.0, 4.0), (2.0, 4.0, 2.0, 0.0)];
    let ans = cgl_4_c(n, xy, q, p);
    assert_eq!(ans, vec![2.0.into(), 4.0.into()]);
}

/// https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_4_A
pub fn cgl_4_a(_n: usize, xy: &[(f64, f64)]) -> Vec<(FValue, FValue)> {
    let points = xy
        .iter()
        .cloned()
        .map(|(x, y)| Vector::new(x, y))
        .collect::<Vec<_>>();
    let convex_hull = Polygon::convex_hull(points, true);
    let mut poly = Polygon::new(convex_hull.nodes.into_iter().map(Vector::swap).collect());
    // yについて正規化
    poly.normalize();
    let ans = Polygon::new(poly.nodes.into_iter().map(Vector::swap).collect());
    ans.nodes.iter().map(|v| (v.x, v.y)).collect()
}

#[test]
fn test_cgl_4_a() {
    let n = 7;
    let xy = vec![
        (2.0, 1.0),
        (0.0, 0.0),
        (1.0, 2.0),
        (2.0, 2.0),
        (4.0, 2.0),
        (1.0, 3.0),
        (3.0, 3.0),
    ];
    let ans = cgl_4_a(n, &xy);
    assert_eq!(5, ans.len());
    assert_eq!(
        ans,
        vec![(0, 0), (2, 1), (4, 2), (3, 3), (1, 3)]
            .into_iter()
            .map(|(x, y)| (x.into(), y.into()))
            .collect::<Vec<(FValue, FValue)>>()
    )
}
