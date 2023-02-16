//! # 二次元平面(浮動小数点数)
use prelude::*;

#[snippet(name = "plane-float", doc_hidden)]
/// 点
#[derive(Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[snippet(name = "plane-float", doc_hidden)]
#[derive(Copy, Clone)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

#[snippet(name = "plane-float", doc_hidden)]
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    /// 偏角を求める(radian)
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

    /// ## 外積
    pub fn cross(p: Self, q: Self) -> f64 {
        p.x * q.y - p.y * q.x
    }

    /// ## 内積
    pub fn dot(p: Self, q: Self) -> f64 {
        p.x * q.x + p.y * q.y
    }

    /// ## ノルム
    pub fn norm(self) -> f64 {
        Self::dot(self, self)
    }

    /// 大きさを求める
    pub fn abs(self) -> f64 {
        self.norm().sqrt()
    }
}

#[snippet(name = "plane-float", doc_hidden)]
#[derive(Copy, Clone)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

#[snippet(name = "plane-float", doc_hidden)]
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
        Line::cross_points(p1p2, p2p3)
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other;
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = *self - other;
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other;
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl Div<f64> for Point {
    type Output = Point;
    fn div(self, rhs: f64) -> Point {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl DivAssign<f64> for Point {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "x:{} y:{}", self.x, self.y)
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Point: (x: {}, y: {})", self.x, self.y)
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl Line {
    pub fn new(p: Point, q: Point) -> Line {
        Line { p1: p, p2: q }
    }

    pub fn cross(l: &Self, m: &Self) -> f64 {
        Point::cross(m.p2 - m.p1, l.p2 - l.p1)
    }

    /// 交点を求める
    pub fn cross_points(l: Self, m: Self) -> Option<Point> {
        let d = Self::cross(&l, &m);
        if d.abs() < std::f64::EPSILON {
            None
        } else {
            Some(l.p1 + (l.p2 - l.p1) * Point::cross(m.p2 - m.p1, m.p2 - l.p1) / d)
        }
    }

    pub fn cross_points_as_segment(l: Self, m: Self) -> Option<Point> {
        let p = Self::cross_points(l, m);
        p.filter(|&p| {
            (p - l.p1).abs() + (l.p2 - p).abs() - (l.p2 - l.p1).abs() < std::f64::EPSILON
                && (p - m.p1).abs() + (m.p2 - p).abs() - (m.p2 - m.p1).abs() < std::f64::EPSILON
        })
    }

    /// xを与えたときのyの値を求める
    pub fn y(self, x: f64) -> Option<f64> {
        if (self.p1.x - self.p2.x).abs() < std::f64::EPSILON {
            None
        } else {
            Some(self.p1.y + (self.p2.y - self.p1.y) / (self.p2.x - self.p1.x) * (x - self.p1.x))
        }
    }

    /// yを与えたときのxの値を求める
    pub fn x(self, y: f64) -> Option<f64> {
        if (self.p1.y - self.p2.y).abs() < std::f64::EPSILON {
            None
        } else {
            Some(self.p1.x + (self.p2.x - self.p1.x) / (self.p2.y - self.p1.y) * (y - self.p1.y))
        }
    }

    /// 直線と点の距離
    pub fn distance(self, p: Point) -> f64 {
        if (self.p1.x - self.p2.x).abs() < std::f64::EPSILON {
            return (p.x - self.p1.x).abs();
        }
        if (self.p1.y - self.p2.y).abs() < std::f64::EPSILON {
            return (p.y - self.p1.y).abs();
        }
        let l = Line::new(p, p + (self.p2 - self.p1).rot90());
        match Self::cross_points(self, l) {
            Some(cp) => (p - cp).abs(),
            None => 0.0,
        }
    }
}

#[snippet(name = "plane-float", doc_hidden)]
impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} - {}", self.p1, self.p2)
    }
}

///////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cross_point() {
        let l1 = Line::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
        let l2 = Line::new(Point::new(0.0, 5.0), Point::new(5.0, 0.0));

        let cp = Line::cross_points(l1, l2).unwrap();
        assert_eq!(cp.x, 2.5);
        assert_eq!(cp.y, 2.5);
    }

    #[test]
    fn cross_point_as_segment() {
        let l1 = Line::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
        let l2 = Line::new(Point::new(0.0, 0.0), Point::new(2.49, 2.49));
        let l3 = Line::new(Point::new(2.51, 2.51), Point::new(5.0, 5.0));
        let l4 = Line::new(Point::new(5.0, 5.0), Point::new(2.51, 2.51));
        let m = Line::new(Point::new(0.0, 5.0), Point::new(5.0, 0.0));

        assert_eq!(
            Some(Point::new(2.5, 2.5)),
            Line::cross_points_as_segment(l1, m)
        );
        assert_eq!(
            Some(Point::new(2.5, 2.5)),
            Line::cross_points_as_segment(m, l1)
        );
        assert_eq!(None, Line::cross_points_as_segment(l2, m));
        assert_eq!(None, Line::cross_points_as_segment(m, l2));
        assert_eq!(None, Line::cross_points_as_segment(m, l3));
        assert_eq!(None, Line::cross_points_as_segment(l3, m));
        assert_eq!(None, Line::cross_points_as_segment(l4, m));
        assert_eq!(None, Line::cross_points_as_segment(m, l4));

        let l1 = Line::new(Point::new(0.0, 0.0), Point::new(5.0, 0.0));
        let m1 = Line::new(Point::new(2.5, 0.01), Point::new(2.5, 1.0));
        let m2 = Line::new(Point::new(2.5, 0.0), Point::new(2.5, 1.0));
        let m3 = Line::new(Point::new(2.5, -0.01), Point::new(2.5, 1.0));
        assert_eq!(None, Line::cross_points_as_segment(l1, m1));
        assert_eq!(
            Some(Point::new(2.5, 0.0)),
            Line::cross_points_as_segment(l1, m2)
        );
        assert_eq!(
            Some(Point::new(2.5, 0.0)),
            Line::cross_points_as_segment(l1, m3)
        );
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
}
