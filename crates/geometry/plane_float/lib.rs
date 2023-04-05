//! # 二次元平面(浮動小数点数)
use min_max_macro::min;
use prelude::*;

#[snippet(name = "plane-float", doc_hidden)]
pub use plane_float_impl::{Circle, ClockwiseDirection, Line, Point, Polygon, Segment, Triangle};
#[snippet(name = "plane-float", doc_hidden)]
mod plane_float_impl {
    use super::{
        min, Add, AddAssign, Debug, Display, Div, DivAssign, Formatter, Mul, MulAssign, Neg, Sub,
        SubAssign,
    };

    const EPS: f64 = std::f64::EPSILON;

    /// 点
    #[derive(Copy, Clone, PartialOrd)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    /// # Tuple(x, y)から生成
    impl From<(f64, f64)> for Point {
        fn from(value: (f64, f64)) -> Self {
            Self::new(value.0, value.1)
        }
    }

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            let p = *self - *other;
            p.x.abs() < EPS && p.y.abs() < EPS
        }
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }

        /// # 偏角を求める($0.0 <= rad <= 2\pi$)
        /// 原点だった場合はNone
        pub fn declination(&self) -> Option<f64> {
            use std::f64::consts::PI;
            if self.x == 0.0 {
                if self.y == 0.0 {
                    None
                } else if self.y > 0.0 {
                    Some(PI / 2.0)
                } else {
                    Some(PI * 3.0 / 2.0)
                }
            } else {
                Some((self.y).atan2(self.x).rem_euclid(PI * 2.0))
            }
        }

        /// 原点を軸にradian回転させる
        pub fn rot(self, radian: f64) -> Point {
            Point::new(
                radian.cos() * self.x - radian.sin() * self.y,
                radian.sin() * self.x + radian.cos() * self.y,
            )
        }

        /// ## 原点を軸にpi/2回転させる
        pub fn rot90(self) -> Point {
            Point::new(-self.y, self.x)
        }

        /// ## x軸に対して反転
        pub fn conj(self) -> Point {
            Point::new(self.x, -self.y)
        }

        /// ## 原点からのベクトルとして見た時の外積
        pub fn cross(p: Self, q: Self) -> f64 {
            p.x * q.y - p.y * q.x
        }

        /// ## 原点からのベクトルとして見た時の内積
        pub fn dot(p: Self, q: Self) -> f64 {
            p.x * q.x + p.y * q.y
        }

        /// ## 原点からのベクトルとして見た時のノルム
        pub fn norm(self) -> f64 {
            Self::dot(self, self)
        }

        /// ## 原点からのベクトルとして見た時の大きさ
        pub fn abs(self) -> f64 {
            self.norm().sqrt()
        }

        /// # 2点間の距離
        pub fn distance(&self, another: &Self) -> f64 {
            (*self - *another).abs()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    /// 3点A,B,Cの位置関係
    pub enum ClockwiseDirection {
        /// 時計回り
        Clockwise,
        /// 反時計回り
        CounterClockwise,
        /// 3点が一直線上にあり、C->A->Bの順に並んでいる
        OneLineCAB,
        /// 3点が一直線上にあり、A->B->Cの順に並んでいる
        OneLineABC,
        /// 3点が一直線上にあり、A->C->Bの順に並んでいる
        OneLineACB,
    }

    impl ClockwiseDirection {
        /// # 3点の関係性を調べる
        pub fn direction(a: Point, b: Point, c: Point) -> Self {
            let (b, c) = (b - a, c - a);
            let cross = Point::cross(b, c);
            if cross > EPS {
                Self::CounterClockwise
            } else if cross < -EPS {
                Self::Clockwise
            } else if Point::dot(b, c) < 0.0 {
                Self::OneLineCAB
            } else if b.norm() < c.norm() {
                Self::OneLineABC
            } else {
                Self::OneLineACB
            }
        }

        /// # 同じ側にないことを調べる
        /// この関数は同じ3点について調べた結果であることを保証しない
        pub fn is_counterside(self, other: Self) -> bool {
            use ClockwiseDirection::*;
            self == OneLineACB
                || other == OneLineACB
                || (self, other) == (OneLineABC, OneLineCAB)
                || (other, self) == (OneLineABC, OneLineCAB)
                || (self, other) == (Clockwise, CounterClockwise)
                || (other, self) == (Clockwise, CounterClockwise)
        }
    }

    /// # 原点に対称な点
    impl Neg for Point {
        type Output = Point;
        fn neg(self) -> Self::Output {
            Point::new(-self.x, -self.y)
        }
    }

    impl Add<Point> for Point {
        type Output = Point;
        fn add(self, rhs: Point) -> Point {
            Point::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl AddAssign<Point> for Point {
        fn add_assign(&mut self, other: Point) {
            *self = *self + other;
        }
    }

    impl Sub<Point> for Point {
        type Output = Point;
        fn sub(self, rhs: Point) -> Point {
            Point::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl SubAssign<Point> for Point {
        fn sub_assign(&mut self, other: Point) {
            *self = *self - other;
        }
    }

    impl Mul<f64> for Point {
        type Output = Point;
        fn mul(self, rhs: f64) -> Point {
            Point::new(self.x * rhs, self.y * rhs)
        }
    }

    impl MulAssign<f64> for Point {
        fn mul_assign(&mut self, other: f64) {
            *self = *self * other;
        }
    }

    impl Div<f64> for Point {
        type Output = Point;
        fn div(self, rhs: f64) -> Point {
            Point::new(self.x / rhs, self.y / rhs)
        }
    }

    impl DivAssign<f64> for Point {
        fn div_assign(&mut self, other: f64) {
            *self = *self / other;
        }
    }

    impl Display for Point {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "x:{} y:{}", self.x, self.y)
        }
    }

    impl Debug for Point {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Point: (x: {}, y: {})", self.x, self.y)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Line {
        pub p1: Point,
        pub p2: Point,
    }

    impl From<Segment> for Line {
        fn from(value: Segment) -> Self {
            Self {
                p1: value.p1,
                p2: value.p2,
            }
        }
    }

    impl Line {
        pub fn new(p: Point, q: Point) -> Line {
            Line { p1: p, p2: q }
        }

        /// # 2直線の内積
        pub fn dot(l: Self, m: Self) -> f64 {
            Point::dot(m.p2 - m.p1, l.p2 - l.p1)
        }

        /// # 2直線の外積
        pub fn cross(l: Self, m: Self) -> f64 {
            Point::cross(m.p2 - m.p1, l.p2 - l.p1)
        }

        /// # 2直線の直交判定(内積が0)
        pub fn is_orthogonal(l: Self, r: Self) -> bool {
            Self::dot(l, r).abs() < EPS
        }

        /// # 2直線の平行判定(外積が0)
        pub fn is_parallel(l: Self, r: Self) -> bool {
            Self::cross(l, r).abs() < EPS
        }

        /// # 2直線の交点
        pub fn cross_point(l: Self, m: Self) -> Option<Point> {
            let d = Self::cross(l, m);
            if d.abs() < EPS {
                None
            } else {
                Some(l.p1 + (l.p2 - l.p1) * Point::cross(m.p2 - m.p1, m.p2 - l.p1) / d)
            }
        }

        /// # xを与えたときのyの値を求める
        pub fn y(self, x: f64) -> Option<f64> {
            if (self.p1.x - self.p2.x).abs() < EPS {
                None
            } else {
                Some(
                    self.p1.y + (self.p2.y - self.p1.y) / (self.p2.x - self.p1.x) * (x - self.p1.x),
                )
            }
        }

        /// # yを与えたときのxの値を求める
        pub fn x(self, y: f64) -> Option<f64> {
            if (self.p1.y - self.p2.y).abs() < EPS {
                None
            } else {
                Some(
                    self.p1.x + (self.p2.x - self.p1.x) / (self.p2.y - self.p1.y) * (y - self.p1.y),
                )
            }
        }

        /// # 直線と点の距離
        pub fn distance(self, p: Point) -> f64 {
            if (self.p1.x - self.p2.x).abs() < EPS {
                return (p.x - self.p1.x).abs();
            }
            if (self.p1.y - self.p2.y).abs() < EPS {
                return (p.y - self.p1.y).abs();
            }
            let l = Line::new(p, p + (self.p2 - self.p1).rot90());
            match Self::cross_point(self, l) {
                Some(cp) => (p - cp).abs(),
                None => 0.0,
            }
        }

        /// # 射影
        /// 点から直線に引いた垂線の足
        pub fn projection(self, p: Point) -> Point {
            let t = Point::dot(p - self.p1, self.p1 - self.p2) / Point::norm(self.p1 - self.p2);
            self.p1 + (self.p1 - self.p2) * t
        }

        /// # 反射
        /// 直線を対称軸として点$P$と線対称にある位置の点
        pub fn reflection(self, p: Point) -> Point {
            p + (self.projection(p) - p) * 2.0
        }
    }

    impl Display for Line {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{} - {}", self.p1, self.p2)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Segment {
        /// 端点1
        pub p1: Point,
        /// 端点2
        pub p2: Point,
    }

    impl From<Line> for Segment {
        fn from(value: Line) -> Self {
            Self {
                p1: value.p1,
                p2: value.p2,
            }
        }
    }

    impl Segment {
        pub fn new(p1: Point, p2: Point) -> Self {
            Self { p1, p2 }
        }

        /// # 線分同士の交点
        pub fn cross_point(l: Self, m: Self) -> Option<Point> {
            if Self::is_intersect(l, m) {
                Line::cross_point(l.into(), m.into())
            } else {
                None
            }
        }

        /// # 線分の共有点の存在
        pub fn is_intersect(l: Self, m: Self) -> bool {
            ClockwiseDirection::direction(m.p1, m.p2, l.p1)
                .is_counterside(ClockwiseDirection::direction(m.p1, m.p2, l.p2))
                && ClockwiseDirection::direction(l.p1, l.p2, m.p1)
                    .is_counterside(ClockwiseDirection::direction(l.p1, l.p2, m.p2))
        }

        /// # 線分と点の距離
        /// $\angle\mathrm{P_2P_1P}$が鈍角 $\Leftrightarrow$ $\mathrm{P_1P}$が距離
        /// $\angle\mathrm{P_1P_2P}$が鈍角 $\Leftrightarrow$ $\mathrm{P_2P}$が距離
        /// 上記以外は垂線の足が距離
        pub fn distance_to_point(s: Self, p: Point) -> f64 {
            if Point::dot(s.p2 - s.p1, p - s.p1) < 0.0 {
                (p - s.p1).abs()
            } else if Point::dot(s.p1 - s.p2, p - s.p2) < 0.0 {
                (p - s.p2).abs()
            } else {
                Line::from(s).distance(p)
            }
        }

        /// # 線分同士の距離
        pub fn distance(l: Self, m: Self) -> f64 {
            if Self::is_intersect(l, m) {
                0.0
            } else {
                min!(
                    Self::distance_to_point(l, m.p1),
                    Self::distance_to_point(l, m.p2),
                    Self::distance_to_point(m, l.p1),
                    Self::distance_to_point(m, l.p2),
                )
            }
        }
    }

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

    /// # 多角形
    #[derive(Clone)]
    pub struct Polygon {
        /// 頂点は反時計回りの順
        nodes: Vec<Point>,
    }

    impl From<&[(f64, f64)]> for Polygon {
        fn from(nodes: &[(f64, f64)]) -> Self {
            Self {
                nodes: nodes.iter().map(|&p| Point::from(p)).collect(),
            }
        }
    }

    impl Polygon {
        pub fn new(nodes: Vec<Point>) -> Self {
            Self { nodes }
        }

        pub fn area(&self) -> f64 {
            let mut res = 0.0;
            for i in 0..self.nodes.len() {
                res += Point::cross(self.nodes[i], self.nodes[(i + 1) % self.nodes.len()]);
            }
            res * 0.5
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cross_point() {
        let l1 = Line::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
        let l2 = Line::new(Point::new(0.0, 5.0), Point::new(5.0, 0.0));

        let cp = Line::cross_point(l1, l2).unwrap();
        assert_eq!(cp.x, 2.5);
        assert_eq!(cp.y, 2.5);
    }

    #[test]
    fn cross_point_as_segment() {
        let l1 = Segment::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
        let l2 = Segment::new(Point::new(0.0, 0.0), Point::new(2.49, 2.49));
        let l3 = Segment::new(Point::new(2.51, 2.51), Point::new(5.0, 5.0));
        let l4 = Segment::new(Point::new(5.0, 5.0), Point::new(2.51, 2.51));
        let m = Segment::new(Point::new(0.0, 5.0), Point::new(5.0, 0.0));

        assert_eq!(Some(Point::new(2.5, 2.5)), Segment::cross_point(l1, m));
        assert_eq!(Some(Point::new(2.5, 2.5)), Segment::cross_point(m, l1));
        assert_eq!(None, Segment::cross_point(l2, m));
        assert_eq!(None, Segment::cross_point(m, l2));
        assert_eq!(None, Segment::cross_point(m, l3));
        assert_eq!(None, Segment::cross_point(l3, m));
        assert_eq!(None, Segment::cross_point(l4, m));
        assert_eq!(None, Segment::cross_point(m, l4));

        let l1 = Segment::new(Point::new(0.0, 0.0), Point::new(5.0, 0.0));
        let m1 = Segment::new(Point::new(2.5, 0.01), Point::new(2.5, 1.0));
        let m2 = Segment::new(Point::new(2.5, 0.0), Point::new(2.5, 1.0));
        let m3 = Segment::new(Point::new(2.5, -0.01), Point::new(2.5, 1.0));
        assert_eq!(None, Segment::cross_point(l1, m1));
        assert_eq!(Some(Point::new(2.5, 0.0)), Segment::cross_point(l1, m2));
        assert_eq!(Some(Point::new(2.5, 0.0)), Segment::cross_point(l1, m3));
    }

    #[test]
    fn distance() {
        let l = Line::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
        let p = Point::new(5.0, 0.0);

        let d = l.distance(p);
        assert_eq!(d, (2.5f64 * 2.5 + 2.5 * 2.5).sqrt());

        let l = Line::new(Point::new(0.0, 0.0), Point::new(5.0, 0.0));
        let p = Point::new(3.0, 2.0);
        assert_eq!(l.distance(p), 2.0);

        let l = Line::new(Point::new(0.0, 0.0), Point::new(0.0, 5.0));
        let p = Point::new(3.0, 2.0);
        assert_eq!(l.distance(p), 3.0);
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
    fn xy() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(5.0, 10.0);
        let line = Line::new(p1, p2);
        assert_eq!(line.x(2.0), Some(1.0));
        assert_eq!(line.y(1.0), Some(2.0));
    }

    #[test]
    fn declination() {
        let p = Point::new(0.0, 0.0);
        assert_eq!(None, p.declination());

        let p = Point::new(1.0, 0.0);
        assert_eq!(Some(std::f64::consts::PI * 0.0 / 4.0), p.declination());

        let p = Point::new(1.0, 1.0);
        assert_eq!(Some(std::f64::consts::PI * 1.0 / 4.0), p.declination());

        let p = Point::new(0.0, 1.0);
        assert_eq!(Some(std::f64::consts::PI * 2.0 / 4.0), p.declination());

        let p = Point::new(-1.0, 1.0);
        assert_eq!(Some(std::f64::consts::PI * 3.0 / 4.0), p.declination());

        let p = Point::new(-1.0, 0.0);
        assert_eq!(Some(std::f64::consts::PI * 4.0 / 4.0), p.declination());

        let p = Point::new(-1.0, -1.0);
        assert_eq!(Some(std::f64::consts::PI * 5.0 / 4.0), p.declination());

        let p = Point::new(0.0, -1.0);
        assert_eq!(Some(std::f64::consts::PI * 6.0 / 4.0), p.declination());

        let p = Point::new(1.0, -1.0);
        assert_eq!(Some(std::f64::consts::PI * 7.0 / 4.0), p.declination());

        let p = Point::new(1.0, -0.0);
        assert_eq!(Some(std::f64::consts::PI * 0.0 / 4.0), p.declination());
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
}
