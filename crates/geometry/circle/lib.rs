//! # 円
use float_value::{FValue, EPS};
use plane_float::{Line, Vector};
use prelude::*;

#[snippet(name = "circle", doc_hidden)]
pub use circle_impl::{Circle, CircleIntersection, Triangle};
#[snippet(name = "circle", doc_hidden)]
#[rustfmt::skip]
mod circle_impl {
    use super::{FValue, Line, Vector, EPS};
    #[derive(Copy, Clone)]
    pub struct Triangle {
        p1: Vector,
        p2: Vector,
        p3: Vector,
    }

    impl Triangle {
        pub fn new(p1: Vector, p2: Vector, p3: Vector) -> Triangle {
            Triangle { p1, p2, p3 }
        }

        /// # 正の面積を持つ三角形かどうか判定する
        fn is_valid(&self) -> bool {
            let a = self.p2 - self.p1;
            let b = self.p3 - self.p1;
            (a.x.0 * b.y.0 - a.y.0 * b.x.0).abs() >= EPS
        }

        /// # 内接円
        pub fn inner_circle(&self) -> Option<Circle> {
            self.inner_center().map(|c| Circle {
                center: c,
                radius: self.inner_circle_radius(),
            })
        }

        /// # 内心
        pub fn inner_center(&self) -> Option<Vector> {
            if self.is_valid() {
                let p1p2 = self.p1.distance(&self.p2);
                let p2p3 = self.p2.distance(&self.p3);
                let p3p1 = self.p3.distance(&self.p1);
                Some((self.p3 * p1p2 + self.p1 * p2p3 + self.p2 * p3p1) / (p1p2 + p2p3 + p3p1))
            } else {
                None
            }
        }

        /// # 内接円の半径
        fn inner_circle_radius(&self) -> FValue {
            let a = (self.p1 - self.p2).abs();
            let b = (self.p2 - self.p3).abs();
            let c = (self.p3 - self.p1).abs();
            let s = self.area();
            2.0 * s / (a + b + c)
        }

        /// # 面積
        pub fn area(&self) -> f64 {
            let a = self.p2 - self.p1;
            let b = self.p3 - self.p1;
            (a.x.0 * b.y.0 - a.y.0 * b.x.0).abs() / 2.0
        }

        /// # 外接円
        pub fn circumscribed_circle(&self) -> Option<Circle> {
            self.circumcenter().map(|c| Circle {
                center: c,
                radius: c.distance(&self.p1),
            })
        }

        /// 外心を求める
        pub fn circumcenter(&self) -> Option<Vector> {
            let p1p2 = Line::new(
                (self.p1 + self.p2) / 2.0,
                (self.p1 + self.p2) / 2.0 + (self.p1 - self.p2).rot90(),
            );
            let p2p3 = Line::new(
                (self.p2 + self.p3) / 2.0,
                (self.p2 + self.p3) / 2.0 + (self.p2 - self.p3).rot90(),
            );
            Line::cross_point(p1p2, p2p3)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Circle {
        pub center: Vector,
        pub radius: FValue,
    }

    impl Circle {
        pub fn new(center_x: f64, center_y: f64, radius: f64) -> Self {
            Self {
                center: Vector::new(center_x, center_y),
                radius: radius.into(),
            }
        }

        /// # 二つの円の距離
        /// 交わっているときは0、片方の円が内包するときは負の値を返す
        pub fn distance(&self, another: &Self) -> FValue {
            let d = self.center.distance(&another.center);
            if d > self.radius + another.radius {
                d - (self.radius + another.radius)
            } else if d < self.radius - another.radius {
                d - (self.radius - another.radius)
            } else {
                0.0.into()
            }
        }

        /// # 二つの円の交点
        /// 順序は不定
        pub fn cross_points(&self, another: &Self) -> Vec<Vector> {
            if self.distance(another) == 0.0.into() {
                let p = another.center - self.center;
                let rad2 = self.radius * self.radius;
                let xxyy = p.x * p.x + p.y * p.y;
                let a = (xxyy + rad2 - (another.radius * another.radius)).0 / 2.0;
                let sq = (xxyy * rad2 - a * a).0.sqrt();
                if sq < std::f64::EPSILON {
                    vec![Vector::new(p.x * a / xxyy, a * p.y / xxyy)]
                } else {
                    vec![
                        Vector::new((a * p.x + p.y * sq) / xxyy, (a * p.y - p.x * sq) / xxyy),
                        Vector::new((a * p.x - p.y * sq) / xxyy, (a * p.y + p.x * sq) / xxyy),
                    ]
                }
            } else {
                Vec::new()
            }
        }

        /// # 円と直線の交点
        /// - 円と直線が遠いとき: 交点なし
        /// - 円と直線が接するとき: 円の中心から直線への射影
        /// - 円と直線が交わるとき: 円の中心から直線への射影から、法線ベクトルの$ph$倍
        ///   - $ph$は三平方の定理で求められる
        pub fn cross_point_to_line(&self, line: &Line) -> Vec<Vector> {
            let d = line.distance(self.center);
            let s = d - self.radius;
            if s > FValue::eps() {
                Vec::new()
            } else if s < -FValue::eps() {
                let proj = line.projection(self.center);
                // 法線ベクトル
                let e = line.normalize();
                let ph = (self.radius * self.radius - d * d).sqrt();
                vec![proj + e * ph, proj - e * ph]
            } else {
                vec![line.projection(self.center)]
            }
        }

        /// # 2円の交点
        pub fn cross_point_to_circle(&self, circle: &Circle) -> Vec<Vector> {
            use CircleIntersection::*;
            match CircleIntersection::intersect(self, circle) {
                NotCross => Vec::new(),
                Circumscribed => {
                    let t = self.radius / (self.radius + circle.radius);
                    vec![self.center + (circle.center - self.center) * t]
                }
                Intersect => {
                    // 求める2点は ax + by + c = 0 を通る
                    let (a, b, c) = (
                        (circle.center.x - self.center.x) * 2.0,
                        (circle.center.y - self.center.y) * 2.0,
                        (self.center.x + circle.center.x) * (self.center.x - circle.center.x)
                            + (self.center.y + circle.center.y) * (self.center.y - circle.center.y)
                            + (circle.radius + self.radius) * (circle.radius - self.radius),
                    );
                    let l = if a == 0.0.into() {
                        Line::new(Vector::new(0.0, -c / b), Vector::new(1.0, -c / b))
                    } else if b == 0.0.into() {
                        Line::new(Vector::new(-c / a, 0.0), Vector::new(-c / a, 1.0))
                    } else if c == 0.0.into() {
                        Line::new(Vector::new(0.0, 0.0), Vector::new(1.0, -a / b))
                    } else {
                        Line::new(Vector::new(0.0, -c / b), Vector::new(-c / a, 0.0))
                    };
                    self.cross_point_to_line(&l)
                }
                Inscribed => vec![if circle.radius < self.radius - EPS {
                    self.center
                        + (circle.center - self.center)
                            * (self.radius / self.center.distance(&circle.center))
                } else {
                    self.center
                        + (circle.center - self.center)
                            * (circle.radius / self.center.distance(&circle.center))
                }],
                Included => Vec::new(),
            }
        }

        /// # 点$P$を通る円への接線における接点(2つorなし)
        pub fn tangent(&self, p: Vector) -> Vec<Vector> {
            self.cross_point_to_circle(&Self {
                center: p,
                radius: ((self.center - p).norm() - self.radius * self.radius).sqrt(),
            })
        }
    }

    /// # 二円の位置関係
    #[derive(Clone, Debug)]
    pub enum CircleIntersection {
        // 遠い(共有点なし)
        NotCross,
        // 外接する
        Circumscribed,
        // 交わる
        Intersect,
        // 内接する
        Inscribed,
        // 内包する(共有点なし)
        Included,
    }

    impl CircleIntersection {
        pub fn intersect(c1: &Circle, c2: &Circle) -> Self {
            let d = c1.center.distance(&c2.center);
            if d > c1.radius + c2.radius + EPS {
                CircleIntersection::NotCross
            } else if (d - (c1.radius + c2.radius)).abs() < FValue::eps() {
                CircleIntersection::Circumscribed
            } else if (d - (c1.radius - c2.radius).abs()).abs() < FValue::eps() {
                CircleIntersection::Inscribed
            } else if d < (c1.radius - c2.radius).abs() - EPS {
                CircleIntersection::Included
            } else {
                CircleIntersection::Intersect
            }
        }
    }
}

#[test]
fn circumcenter() {
    let p1 = Vector::new(0.0, 0.0);
    let p2 = Vector::new(4.0, 0.0);
    let p3 = Vector::new(2.0, 2.0 * 3.0f64.sqrt());
    let tr = Triangle::new(p1, p2, p3);
    let cc = tr.circumcenter().unwrap();
    assert_eq!(cc.x.0, 2.0);
    assert_eq!(cc.y.0, 2.0f64 / 3.0f64.sqrt());
}

#[test]
fn circle_distance() {
    let a = Circle::new(0.0, 0.0, 1.0);
    let b = Circle::new(1.0, 1.0, 1.0);
    assert_eq!(0.0, a.distance(&b).0);

    let c = Circle::new(2.0, 0.0, 1.0);
    assert_eq!(0.0, a.distance(&c).0);

    let d = Circle::new(3.0, 0.0, 1.0);
    assert_eq!(1.0, a.distance(&d).0);

    let e = Circle::new(0.0, 0.0, 0.5);
    assert_eq!(-0.5, a.distance(&e).0);
}

#[test]
fn circle_cross_points() {
    let a = Circle::new(0.0, 0.0, 1.0);
    let b = Circle::new(1.0, 1.0, 1.0);
    let mut expect = vec![Vector::new(0.0, 1.0), Vector::new(1.0, 0.0)];
    let mut result = a.cross_points(&b);
    expect.sort_by(|a, b| a.partial_cmp(b).unwrap());
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(expect, result);

    let c = Circle::new(2.0, 0.0, 1.0);
    assert_eq!(vec![Vector::new(1.0, 0.0)], a.cross_points(&c));

    let d = Circle::new(3.0, 0.0, 1.0);
    assert_eq!(Vec::<Vector>::new(), a.cross_points(&d));

    let e = Circle::new(0.0, 0.0, 0.5);
    assert_eq!(Vec::<Vector>::new(), a.cross_points(&e));
}

/// <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/7/CGL_7_C>
pub fn cgl_7_c(xy: &[(f64, f64)]) -> (f64, f64, f64) {
    let p: Vec<_> = xy.iter().cloned().map(Vector::from).collect();
    let tri = Triangle::new(p[0], p[1], p[2]);
    let circle = tri.circumscribed_circle().unwrap();
    (circle.center.x.0, circle.center.y.0, circle.radius.0)
}

#[test]
fn test_cgl_7_c() {
    let ans = cgl_7_c(&vec![(1.0, -2.0), (3.0, 2.0), (-2.0, 0.0)]);
    assert_eq!(
        ans,
        (0.625.into(), 0.6875.into(), 2.71353666826155124291.into())
    )
}

pub fn cgl_7_f(p: (f64, f64), c: (f64, f64), r: f64) -> ((FValue, FValue), (FValue, FValue)) {
    let p = p.into();
    let c = Circle::new(c.0, c.1, r);

    let mut ans = c.tangent(p);
    ans.sort();
    (ans[0].into(), ans[1].into())
}

#[test]
fn test_cgl_7_f() {
    assert_eq!(
        cgl_7_f((0.0, 0.0), (2.0, 2.0), 2.0),
        ((0.0.into(), 2.0.into()), (2.0.into(), 0.0.into()))
    );
    assert_eq!(
        cgl_7_f((-3.0, 0.0), (2.0, 2.0), 2.0),
        (
            (0.6206896552.into(), 3.4482758621.into()),
            (2.0.into(), 0.0.into())
        )
    )
}
