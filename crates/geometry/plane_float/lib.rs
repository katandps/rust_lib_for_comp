//! # 二次元平面(浮動小数点数)
use min_max_macro::min;
use prelude::*;

#[snippet(name = "plane-float", doc_hidden)]
pub use plane_float_impl::{ClockwiseDirection, Line, Segment, Vector, EPS};
#[snippet(name = "plane-float", doc_hidden)]
mod plane_float_impl {
    use super::{
        min, Add, AddAssign, Debug, Display, Div, DivAssign, Formatter, Mul, MulAssign, Neg, Sub,
        SubAssign,
    };

    pub const EPS: f64 = std::f64::EPSILON;

    /// 点ベクトル
    #[derive(Copy, Clone, Default, PartialOrd)]
    pub struct Vector {
        pub x: f64,
        pub y: f64,
    }

    /// # Tuple(x, y)から生成
    impl From<(f64, f64)> for Vector {
        fn from(value: (f64, f64)) -> Self {
            Self::new(value.0, value.1)
        }
    }

    impl PartialEq for Vector {
        fn eq(&self, other: &Self) -> bool {
            let p = *self - *other;
            p.x.abs() < EPS && p.y.abs() < EPS
        }
    }

    impl Vector {
        pub fn new(x: f64, y: f64) -> Vector {
            Vector { x, y }
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
        pub fn rot(self, radian: f64) -> Vector {
            Vector::new(
                radian.cos() * self.x - radian.sin() * self.y,
                radian.sin() * self.x + radian.cos() * self.y,
            )
        }

        /// # 直線$y=x$について対称に移動する
        pub fn swap(self) -> Vector {
            Vector::new(self.y, self.x)
        }

        /// ## 原点を軸にpi/2回転させる
        pub fn rot90(self) -> Vector {
            Vector::new(-self.y, self.x)
        }

        /// ## x軸に対して反転
        pub fn conj(self) -> Vector {
            Vector::new(self.x, -self.y)
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
        pub fn direction(a: Vector, b: Vector, c: Vector) -> Self {
            let (b, c) = (b - a, c - a);
            let cross = Vector::cross(b, c);
            if cross > EPS {
                Self::CounterClockwise
            } else if cross < -EPS {
                Self::Clockwise
            } else if Vector::dot(b, c) < 0.0 {
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
    impl Neg for Vector {
        type Output = Vector;
        fn neg(self) -> Self::Output {
            Vector::new(-self.x, -self.y)
        }
    }

    impl Add<Vector> for Vector {
        type Output = Vector;
        fn add(self, rhs: Vector) -> Vector {
            Vector::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl AddAssign<Vector> for Vector {
        fn add_assign(&mut self, other: Vector) {
            *self = *self + other;
        }
    }

    impl Sub<Vector> for Vector {
        type Output = Vector;
        fn sub(self, rhs: Vector) -> Vector {
            Vector::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl SubAssign<Vector> for Vector {
        fn sub_assign(&mut self, other: Vector) {
            *self = *self - other;
        }
    }

    impl Mul<f64> for Vector {
        type Output = Vector;
        fn mul(self, rhs: f64) -> Vector {
            Vector::new(self.x * rhs, self.y * rhs)
        }
    }

    impl MulAssign<f64> for Vector {
        fn mul_assign(&mut self, other: f64) {
            *self = *self * other;
        }
    }

    impl Div<f64> for Vector {
        type Output = Vector;
        fn div(self, rhs: f64) -> Vector {
            Vector::new(self.x / rhs, self.y / rhs)
        }
    }

    impl DivAssign<f64> for Vector {
        fn div_assign(&mut self, other: f64) {
            *self = *self / other;
        }
    }

    impl Display for Vector {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "x:{} y:{}", self.x, self.y)
        }
    }

    impl Debug for Vector {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "Point: (x: {}, y: {})", self.x, self.y)
        }
    }

    #[derive(Copy, Clone)]
    pub struct Line {
        pub p1: Vector,
        pub p2: Vector,
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
        pub fn new(p: Vector, q: Vector) -> Line {
            Line { p1: p, p2: q }
        }

        /// # 2直線の内積
        pub fn dot(l: Self, m: Self) -> f64 {
            Vector::dot(m.p2 - m.p1, l.p2 - l.p1)
        }

        /// # 2直線の外積
        pub fn cross(l: Self, m: Self) -> f64 {
            Vector::cross(m.p2 - m.p1, l.p2 - l.p1)
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
        pub fn cross_point(l: Self, m: Self) -> Option<Vector> {
            let d = Self::cross(l, m);
            if d.abs() < EPS {
                None
            } else {
                Some(l.p1 + (l.p2 - l.p1) * Vector::cross(m.p2 - m.p1, m.p2 - l.p1) / d)
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
        pub fn distance(self, p: Vector) -> f64 {
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
        pub fn projection(self, p: Vector) -> Vector {
            let t = Vector::dot(p - self.p1, self.p1 - self.p2) / Vector::norm(self.p1 - self.p2);
            self.p1 + (self.p1 - self.p2) * t
        }

        /// # 反射
        /// 直線を対称軸として点$P$と線対称にある位置の点
        pub fn reflection(self, p: Vector) -> Vector {
            p + (self.projection(p) - p) * 2.0
        }

        /// # 法線ベクトル
        pub fn normal_vector(self) -> Self {
            let a = self.p2 - self.p1;
            Self {
                p1: a.rot90(),
                p2: Vector::default(),
            }
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
        pub p1: Vector,
        /// 端点2
        pub p2: Vector,
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
        pub fn new(p1: Vector, p2: Vector) -> Self {
            Self { p1, p2 }
        }

        /// # 線分同士の交点
        pub fn cross_point(l: Self, m: Self) -> Option<Vector> {
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
        pub fn distance_to_point(s: Self, p: Vector) -> f64 {
            if Vector::dot(s.p2 - s.p1, p - s.p1) < 0.0 {
                (p - s.p1).abs()
            } else if Vector::dot(s.p1 - s.p2, p - s.p2) < 0.0 {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cross_point() {
        let l1 = Line::new(Vector::new(0.0, 0.0), Vector::new(5.0, 5.0));
        let l2 = Line::new(Vector::new(0.0, 5.0), Vector::new(5.0, 0.0));

        let cp = Line::cross_point(l1, l2).unwrap();
        assert_eq!(cp.x, 2.5);
        assert_eq!(cp.y, 2.5);
    }

    #[test]
    fn cross_point_as_segment() {
        let l1 = Segment::new(Vector::new(0.0, 0.0), Vector::new(5.0, 5.0));
        let l2 = Segment::new(Vector::new(0.0, 0.0), Vector::new(2.49, 2.49));
        let l3 = Segment::new(Vector::new(2.51, 2.51), Vector::new(5.0, 5.0));
        let l4 = Segment::new(Vector::new(5.0, 5.0), Vector::new(2.51, 2.51));
        let m = Segment::new(Vector::new(0.0, 5.0), Vector::new(5.0, 0.0));

        assert_eq!(Some(Vector::new(2.5, 2.5)), Segment::cross_point(l1, m));
        assert_eq!(Some(Vector::new(2.5, 2.5)), Segment::cross_point(m, l1));
        assert_eq!(None, Segment::cross_point(l2, m));
        assert_eq!(None, Segment::cross_point(m, l2));
        assert_eq!(None, Segment::cross_point(m, l3));
        assert_eq!(None, Segment::cross_point(l3, m));
        assert_eq!(None, Segment::cross_point(l4, m));
        assert_eq!(None, Segment::cross_point(m, l4));

        let l1 = Segment::new(Vector::new(0.0, 0.0), Vector::new(5.0, 0.0));
        let m1 = Segment::new(Vector::new(2.5, 0.01), Vector::new(2.5, 1.0));
        let m2 = Segment::new(Vector::new(2.5, 0.0), Vector::new(2.5, 1.0));
        let m3 = Segment::new(Vector::new(2.5, -0.01), Vector::new(2.5, 1.0));
        assert_eq!(None, Segment::cross_point(l1, m1));
        assert_eq!(Some(Vector::new(2.5, 0.0)), Segment::cross_point(l1, m2));
        assert_eq!(Some(Vector::new(2.5, 0.0)), Segment::cross_point(l1, m3));
    }

    #[test]
    fn distance() {
        let l = Line::new(Vector::new(0.0, 0.0), Vector::new(5.0, 5.0));
        let p = Vector::new(5.0, 0.0);

        let d = l.distance(p);
        assert_eq!(d, (2.5f64 * 2.5 + 2.5 * 2.5).sqrt());

        let l = Line::new(Vector::new(0.0, 0.0), Vector::new(5.0, 0.0));
        let p = Vector::new(3.0, 2.0);
        assert_eq!(l.distance(p), 2.0);

        let l = Line::new(Vector::new(0.0, 0.0), Vector::new(0.0, 5.0));
        let p = Vector::new(3.0, 2.0);
        assert_eq!(l.distance(p), 3.0);
    }

    #[test]
    fn xy() {
        let p1 = Vector::new(0.0, 0.0);
        let p2 = Vector::new(5.0, 10.0);
        let line = Line::new(p1, p2);
        assert_eq!(line.x(2.0), Some(1.0));
        assert_eq!(line.y(1.0), Some(2.0));
    }

    #[test]
    fn declination() {
        let p = Vector::new(0.0, 0.0);
        assert_eq!(None, p.declination());

        let p = Vector::new(1.0, 0.0);
        assert_eq!(Some(std::f64::consts::PI * 0.0 / 4.0), p.declination());

        let p = Vector::new(1.0, 1.0);
        assert_eq!(Some(std::f64::consts::PI * 1.0 / 4.0), p.declination());

        let p = Vector::new(0.0, 1.0);
        assert_eq!(Some(std::f64::consts::PI * 2.0 / 4.0), p.declination());

        let p = Vector::new(-1.0, 1.0);
        assert_eq!(Some(std::f64::consts::PI * 3.0 / 4.0), p.declination());

        let p = Vector::new(-1.0, 0.0);
        assert_eq!(Some(std::f64::consts::PI * 4.0 / 4.0), p.declination());

        let p = Vector::new(-1.0, -1.0);
        assert_eq!(Some(std::f64::consts::PI * 5.0 / 4.0), p.declination());

        let p = Vector::new(0.0, -1.0);
        assert_eq!(Some(std::f64::consts::PI * 6.0 / 4.0), p.declination());

        let p = Vector::new(1.0, -1.0);
        assert_eq!(Some(std::f64::consts::PI * 7.0 / 4.0), p.declination());

        let p = Vector::new(1.0, -0.0);
        assert_eq!(Some(std::f64::consts::PI * 0.0 / 4.0), p.declination());
    }
}
