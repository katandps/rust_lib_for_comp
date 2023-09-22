//! # 二次元平面(浮動小数点数)
use float_value::FValue;
use min_max_macro::min;
use prelude::*;

#[snippet(name = "plane-float", doc_hidden)]
pub use plane_float_impl::{ClockwiseDirection, Line, Segment, Vector};
#[snippet(name = "plane-float", doc_hidden)]
#[rustfmt::skip]
mod plane_float_impl {
    use super::{
        min, Add, AddAssign, Debug, Display, Div, DivAssign, FValue, Formatter, Mul, MulAssign,
        Neg, Ordering, Sub, SubAssign,
    };

    /// # ベクトル
    /// 始点を$(0, 0)$、終点を$(x, y)$とするベクトル
    #[derive(Copy, Clone, Default, Eq, PartialEq, PartialOrd, Ord)]
    pub struct Vector {
        pub x: FValue,
        pub y: FValue,
    }

    /// # Tuple(x, y)から生成
    impl From<(f64, f64)> for Vector {
        fn from(value: (f64, f64)) -> Self {
            Self::new(value.0, value.1)
        }
    }

    impl Vector {
        pub fn new<X: Into<FValue>, Y: Into<FValue>>(x: X, y: Y) -> Vector {
            Vector {
                x: x.into(),
                y: y.into(),
            }
        }

        /// # 偏角を求める($0.0 <= rad <= 2\pi$)
        /// 原点だった場合はNone
        pub fn declination(&self) -> Option<f64> {
            use std::f64::consts::PI;
            if self.x == 0.0.into() {
                match self.y.cmp(&0.0.into()) {
                    Ordering::Equal => None,
                    Ordering::Greater => Some(PI / 2.0),
                    Ordering::Less => Some(PI * 3.0 / 2.0),
                }
            } else {
                Some((self.y.0).atan2(self.x.0).rem_euclid(PI * 2.0))
            }
        }

        /// 原点を軸にradian回転させる
        pub fn rot(self, radian: f64) -> Vector {
            Vector::new(
                radian.cos() * self.x.0 - radian.sin() * self.y.0,
                radian.sin() * self.x.0 + radian.cos() * self.y.0,
            )
        }

        /// # 直線$y=x$について対称に移動する
        pub fn swap(self) -> Vector {
            Vector::new(self.y, self.x)
        }

        /// ## 始点を軸にpi/2回転させる
        pub fn rot90(self) -> Vector {
            Vector::new(-self.y.0, self.x)
        }

        /// ## x軸に対して反転
        pub fn conj(self) -> Vector {
            Vector::new(self.x, -self.y.0)
        }

        /// ## 外積
        pub fn cross(p: Self, q: Self) -> FValue {
            p.x * q.y - p.y * q.x
        }

        /// ## 内積
        pub fn dot(p: Self, q: Self) -> FValue {
            p.x * q.x + p.y * q.y
        }

        /// ## ノルム
        pub fn norm(self) -> FValue {
            Self::dot(self, self)
        }

        /// ## ベクトルの大きさ
        pub fn abs(self) -> FValue {
            self.norm().sqrt()
        }

        /// # 2点間の距離
        pub fn distance(&self, another: &Self) -> FValue {
            (*self - *another).abs()
        }

        /// # 正規化
        pub fn normalize(self) -> Self {
            self / self.abs()
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
            if cross > FValue::eps() {
                Self::CounterClockwise
            } else if cross < -FValue::eps() {
                Self::Clockwise
            } else if Vector::dot(b, c) < 0.0.into() {
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
            Vector::new(-self.x.0, -self.y.0)
        }
    }

    impl Add<Vector> for Vector {
        type Output = Vector;
        fn add(self, rhs: Vector) -> Vector {
            Vector::new(self.x.0 + rhs.x.0, self.y.0 + rhs.y.0)
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
            Vector::new(self.x.0 - rhs.x.0, self.y.0 - rhs.y.0)
        }
    }

    impl SubAssign<Vector> for Vector {
        fn sub_assign(&mut self, other: Vector) {
            *self = *self - other;
        }
    }

    impl<T: Into<f64>> Mul<T> for Vector {
        type Output = Vector;
        fn mul(self, rhs: T) -> Vector {
            let v = rhs.into();
            Vector::new(self.x.0 * v, self.y.0 * v)
        }
    }

    impl<T: Into<f64>> MulAssign<T> for Vector {
        fn mul_assign(&mut self, other: T) {
            *self = *self * other.into();
        }
    }

    impl<T: Into<f64>> Div<T> for Vector {
        type Output = Vector;
        fn div(self, rhs: T) -> Vector {
            let v = rhs.into();
            Vector::new(self.x / v, self.y / v)
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

    #[derive(Copy, Clone, Debug)]
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

        /// # 方向単位ベクトル
        pub fn normalize(&self) -> Vector {
            (self.p1 - self.p2).normalize()
        }

        /// # 2直線の内積
        pub fn dot(l: Self, m: Self) -> FValue {
            Vector::dot(m.p2 - m.p1, l.p2 - l.p1)
        }

        /// # 2直線の外積
        pub fn cross(l: Self, m: Self) -> FValue {
            Vector::cross(m.p2 - m.p1, l.p2 - l.p1)
        }

        /// # 2直線の直交判定(内積が0)
        pub fn is_orthogonal(l: Self, r: Self) -> bool {
            Self::dot(l, r).abs() < FValue::eps()
        }

        /// # 2直線の平行判定(外積が0)
        pub fn is_parallel(l: Self, r: Self) -> bool {
            Self::cross(l, r).abs() < FValue::eps()
        }

        /// # 2直線の交点
        pub fn cross_point(l: Self, m: Self) -> Option<Vector> {
            let d = Self::cross(l, m);
            if d.abs() < FValue::eps() {
                None
            } else {
                Some(l.p1 + (l.p2 - l.p1) * Vector::cross(m.p2 - m.p1, m.p2 - l.p1) / d)
            }
        }

        /// # xを与えたときのyの値を求める
        pub fn y(self, x: f64) -> Option<FValue> {
            if (self.p1.x - self.p2.x).abs() < FValue::eps() {
                None
            } else {
                Some(
                    self.p1.y + (self.p2.y - self.p1.y) / (self.p2.x - self.p1.x) * (x - self.p1.x),
                )
            }
        }

        /// # yを与えたときのxの値を求める
        pub fn x(self, y: f64) -> Option<FValue> {
            if (self.p1.y - self.p2.y).abs() < FValue::eps() {
                None
            } else {
                Some(
                    self.p1.x + (self.p2.x - self.p1.x) / (self.p2.y - self.p1.y) * (y - self.p1.y),
                )
            }
        }

        /// # 直線と点の距離
        pub fn distance(self, p: Vector) -> FValue {
            if (self.p1.x - self.p2.x).abs() < FValue::eps() {
                return (p.x - self.p1.x).abs();
            }
            if (self.p1.y - self.p2.y).abs() < FValue::eps() {
                return (p.y - self.p1.y).abs();
            }
            let l = Line::new(p, p + (self.p2 - self.p1).rot90());
            match Self::cross_point(self, l) {
                Some(cp) => (p - cp).abs(),
                None => 0.0.into(),
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
        /// 法線ベクトルを求めたのち、単位ベクトルに変換して返す
        pub fn normal_vector(self) -> Vector {
            let a = self.p2 - self.p1;
            a.rot90().normalize()
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
        pub fn distance_to_point(s: Self, p: Vector) -> FValue {
            if Vector::dot(s.p2 - s.p1, p - s.p1) < 0.0.into() {
                (p - s.p1).abs()
            } else if Vector::dot(s.p1 - s.p2, p - s.p2) < 0.0.into() {
                (p - s.p2).abs()
            } else {
                Line::from(s).distance(p)
            }
        }

        /// # 線分同士の距離
        pub fn distance(l: Self, m: Self) -> FValue {
            if Self::is_intersect(l, m) {
                0.0.into()
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
        assert_eq!(cp.x, 2.5.into());
        assert_eq!(cp.y, 2.5.into());
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
        assert_eq!(d, (2.5f64 * 2.5 + 2.5 * 2.5).sqrt().into());

        let l = Line::new(Vector::new(0.0, 0.0), Vector::new(5.0, 0.0));
        let p = Vector::new(3.0, 2.0);
        assert_eq!(l.distance(p), 2.0.into());

        let l = Line::new(Vector::new(0.0, 0.0), Vector::new(0.0, 5.0));
        let p = Vector::new(3.0, 2.0);
        assert_eq!(l.distance(p), 3.0.into());
    }

    #[test]
    fn xy() {
        let p1 = Vector::new(0.0, 0.0);
        let p2 = Vector::new(5.0, 10.0);
        let line = Line::new(p1, p2);
        assert_eq!(line.x(2.0), Some(1.0.into()));
        assert_eq!(line.y(1.0), Some(2.0.into()));
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

/// <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/all/CGL_1_C>
pub fn cgl_1_c(
    p1: (f64, f64),
    p2: (f64, f64),
    _q: usize,
    xy: &[(f64, f64)],
) -> Vec<ClockwiseDirection> {
    let (p1, p2) = (p1.into(), p2.into());
    xy.iter()
        .cloned()
        .map(|(x, y)| {
            let p3 = (x, y).into();
            ClockwiseDirection::direction(p1, p2, p3)
        })
        .collect()
}

#[test]
fn test_cgl_1_c() {
    let ans = cgl_1_c(
        (0.0, 0.0),
        (2.0, 0.0),
        5,
        &vec![
            (-1.0, 1.0),
            (-1.0, -1.0),
            (-1.0, 0.0),
            (0.0, 0.0),
            (3.0, 0.0),
        ],
    );
    assert_eq!(
        ans,
        vec![
            ClockwiseDirection::CounterClockwise,
            ClockwiseDirection::Clockwise,
            ClockwiseDirection::OneLineCAB,
            ClockwiseDirection::OneLineACB,
            ClockwiseDirection::OneLineABC
        ]
    )
}

/// <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_C>
pub fn cgl_2_c(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> (FValue, FValue) {
    if let Some(result) = Segment::cross_point(
        Segment::new(p0.into(), p1.into()),
        Segment::new(p2.into(), p3.into()),
    ) {
        (result.x, result.y)
    } else {
        panic!("line is parallel")
    }
}

#[test]
fn test_cgl_2_c() {
    assert_eq!(
        cgl_2_c((0.0, 0.0), (2.0, 0.0), (1.0, 1.0), (1.0, -1.0)),
        (1.0.into(), 0.0.into())
    );
    assert_eq!(
        cgl_2_c((0.0, 0.0), (1.0, 1.0), (0.0, 1.0), (1.0, 0.0)),
        (0.5.into(), 0.5.into())
    );
    assert_eq!(
        cgl_2_c((0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)),
        (0.5.into(), 0.5.into())
    );
}

/// <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/2/CGL_2_C>
pub fn cgl_2_d(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> FValue {
    Segment::distance(
        Segment::new(p0.into(), p1.into()),
        Segment::new(p2.into(), p3.into()),
    )
}

#[test]
fn test_cgl_2_d() {
    assert_eq!(
        cgl_2_d((0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)),
        1.0.into()
    );
    assert_eq!(
        cgl_2_d((0.0, 0.0), (1.0, 0.0), (2.0, 1.0), (1.0, 2.0)),
        1.4142135624.into()
    );
    assert_eq!(
        cgl_2_d((-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0)),
        0.0.into()
    );
}

pub fn cgl_2_a(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> usize {
    let (l1, l2) = (
        Line::new(p0.into(), p1.into()),
        Line::new(p2.into(), p3.into()),
    );
    if Line::is_parallel(l1, l2) {
        2
    } else if Line::is_orthogonal(l1, l2) {
        1
    } else {
        0
    }
}

#[test]
fn test_cgl_2_a() {
    assert_eq!(cgl_2_a((0.0, 0.0), (3.0, 0.0), (0.0, 2.0), (3.0, 2.0)), 2);
    assert_eq!(cgl_2_a((0.0, 0.0), (3.0, 0.0), (1.0, 1.0), (1.0, 4.0)), 1);
    assert_eq!(cgl_2_a((0.0, 0.0), (3.0, 0.0), (1.0, 1.0), (2.0, 2.0)), 0);
}
