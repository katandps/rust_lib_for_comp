//! # 円
use plane_float::{Line, Point, EPS};
use prelude::*;

#[snippet(name = "circle", doc_hidden)]
pub use circle_impl::{Circle, CircleIntersection, Triangle};
#[snippet(name = "circle", doc_hidden)]
mod circle_impl {
    use super::{Line, Point, EPS};
    #[derive(Copy, Clone)]
    pub struct Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
    }

    impl Triangle {
        pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
            Triangle { p1, p2, p3 }
        }

        /// 内心を求める
        pub fn inner_center(&self) -> Option<Point> {
            let line = Line::new(self.p1, self.p2);
            if line.distance(self.p3) > 0.0 {
                Some((self.p1 + self.p2 + self.p3) / 3.0)
            } else {
                None
            }
        }

        ///内接円の半径
        pub fn inner_circle_radius(&self) -> f64 {
            let a = (self.p1 - self.p2).abs();
            let b = (self.p2 - self.p3).abs();
            let c = (self.p3 - self.p1).abs();
            let s = self.area();
            2.0 * s / (a + b + c)
        }

        /// 面積を求める
        pub fn area(&self) -> f64 {
            let a = self.p2 - self.p1;
            let b = self.p3 - self.p1;
            (a.x * b.y - a.y * b.x).abs() / 2.0
        }

        /// 外心を求める
        pub fn circumcenter(&self) -> Option<Point> {
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
        pub center: Point,
        pub radius: f64,
    }

    impl Circle {
        pub fn new(center_x: f64, center_y: f64, radius: f64) -> Self {
            Self {
                center: Point::new(center_x, center_y),
                radius,
            }
        }

        /// # 二つの円の距離
        /// 交わっているときは0、片方の円が内包するときは負の値を返す
        pub fn distance(&self, another: &Self) -> f64 {
            let d = self.center.distance(&another.center);
            if d > self.radius + another.radius {
                d - (self.radius + another.radius)
            } else if d < (self.radius - another.radius).abs() {
                d - (self.radius - another.radius)
            } else {
                0.0
            }
        }

        pub fn is_intersect(&self, another: &Self) -> CircleIntersection {
            let d = self.center.distance(&another.center);
            if d > self.radius + another.radius + EPS {
                CircleIntersection::NotCross
            } else if (d - (self.radius + another.radius)).abs() < EPS {
                CircleIntersection::Circumscribed
            } else if (d - (self.radius - another.radius).abs()).abs() < EPS {
                CircleIntersection::Inscribed
            } else if d < (self.radius - another.radius).abs() - EPS {
                CircleIntersection::Included
            } else {
                CircleIntersection::Intersect
            }
        }

        /// # 二つの円の交点
        /// 順序は不定
        pub fn cross_points(&self, another: &Self) -> Vec<Point> {
            if self.distance(another) == 0.0 {
                let p = another.center - self.center;
                let rad2 = self.radius * self.radius;
                let xxyy = p.x * p.x + p.y * p.y;
                let a = (xxyy + rad2 - another.radius * another.radius) / 2.0;
                let sq = (xxyy * rad2 - a * a).sqrt();
                if sq < std::f64::EPSILON {
                    vec![Point::new(a * p.x / xxyy, a * p.y / xxyy)]
                } else {
                    vec![
                        Point::new((a * p.x + p.y * sq) / xxyy, (a * p.y - p.x * sq) / xxyy),
                        Point::new((a * p.x - p.y * sq) / xxyy, (a * p.y + p.x * sq) / xxyy),
                    ]
                }
            } else {
                Vec::new()
            }
        }
    }

    /// # 二円の位置関係
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
}

#[test]
fn circumcenter() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(4.0, 0.0);
    let p3 = Point::new(2.0, 2.0 * 3.0f64.sqrt());
    let tr = Triangle::new(p1, p2, p3);
    let cc = tr.circumcenter().unwrap();
    assert_eq!(cc.x, 2.0);
    assert_eq!(cc.y, 2.0f64 / 3.0f64.sqrt());
}

#[test]
fn cicrle_distance() {
    let a = Circle::new(0.0, 0.0, 1.0);
    let b = Circle::new(1.0, 1.0, 1.0);
    assert_eq!(0.0, a.distance(&b));

    let c = Circle::new(2.0, 0.0, 1.0);
    assert_eq!(0.0, a.distance(&c));

    let d = Circle::new(3.0, 0.0, 1.0);
    assert_eq!(1.0, a.distance(&d));

    let e = Circle::new(0.0, 0.0, 0.5);
    assert_eq!(-0.5, a.distance(&e));
}

#[test]
fn circle_cross_points() {
    let a = Circle::new(0.0, 0.0, 1.0);
    let b = Circle::new(1.0, 1.0, 1.0);
    let mut expect = vec![Point::new(0.0, 1.0), Point::new(1.0, 0.0)];
    let mut result = a.cross_points(&b);
    expect.sort_by(|a, b| a.partial_cmp(b).unwrap());
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(expect, result);

    let c = Circle::new(2.0, 0.0, 1.0);
    assert_eq!(vec![Point::new(1.0, 0.0)], a.cross_points(&c));

    let d = Circle::new(3.0, 0.0, 1.0);
    assert_eq!(Vec::<Point>::new(), a.cross_points(&d));

    let e = Circle::new(0.0, 0.0, 0.5);
    assert_eq!(Vec::<Point>::new(), a.cross_points(&e));
}
