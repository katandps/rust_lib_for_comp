#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let n = reader.u();
    let xy = reader.iv2(n);

    let mut ans = 0.0;
    for b in 0..n {
        let bi = Point::new(xy[b].0 as f64, xy[b].1 as f64);
        let mut declinations = Vec::new();
        for i in 0..n {
            if i == b {
                continue;
            }

            let ai = Point::new(xy[i].0 as f64, xy[i].1 as f64);
            let p = ai - bi;
            declinations.push(p.declination().unwrap() * 180.0 / std::f64::consts::PI);
        }
        declinations.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        for i in 0..declinations.len() {
            let mut ok = declinations.len() as i32 - 1;
            let mut ng = 0;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if declinations[mid as usize] > (declinations[i] + 180.0) % 360.0 {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            let a = *partial_min(
                &(declinations[ok as usize] - declinations[i]).abs(),
                &(360.0 - (declinations[ok as usize] - declinations[i]).abs()),
            )
            .unwrap();
            let b = *partial_min(
                &(declinations[ng as usize] - declinations[i]).abs(),
                &(360.0 - (declinations[ng as usize] - declinations[i]).abs()),
            )
            .unwrap();
            ans = *partial_max(&ans, &a).unwrap();
            ans = *partial_max(&ans, &b).unwrap();
        }
    }
    println!("{}", ans);
}

#[allow(unused_imports)]
use geometric::*;

#[allow(dead_code)]
mod geometric {
    use std::f64;
    use std::fmt;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    #[derive(Copy, Clone, PartialEq)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Copy, Clone)]
    pub struct Line {
        pub p1: Point,
        pub p2: Point,
    }

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
                Some(
                    (libm::atan(self.y / self.x) + if self.x > 0.0 { 0.0 } else { PI })
                        .rem_euclid(PI * 2.0),
                )
            }
        }

        /// 原点を軸にradian回転させる
        pub fn rot(self, radian: f64) -> Point {
            Point::new(
                radian.cos() * self.x - radian.sin() * self.y,
                radian.sin() * self.x + radian.cos() * self.y,
            )
        }

        /// 原点を軸にpi/2回転させる
        pub fn rot90(self) -> Point {
            Point::new(-self.y, self.x)
        }

        /// x軸に対して反転
        pub fn conj(self) -> Point {
            Point::new(self.x, -self.y)
        }

        /// 外積を求める
        pub fn cross(p: Self, q: Self) -> f64 {
            p.x * q.y - p.y * q.x
        }

        /// 内積を求める
        pub fn dot(p: Self, q: Self) -> f64 {
            p.x * q.x + p.y * p.y
        }

        /// ノルムを求める
        pub fn norm(self) -> f64 {
            Self::dot(self, self)
        }

        /// 大きさを求める
        pub fn abs(self) -> f64 {
            self.norm().sqrt()
        }

        /// 外心を求める
        pub fn circumcenter(p: Self, q: Self, r: Self) -> Option<Point> {
            let pq = Line::new((p + q) / 2.0, (p + q) / 2.0 + (p - q).rot90());
            let qr = Line::new((q + r) / 2.0, (q + r) / 2.0 + (q - r).rot90());
            Line::cross_points(pq, qr)
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

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x:{} y:{}", self.x, self.y)
        }
    }

    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point: (x: {}, y: {})", self.x, self.y)
        }
    }

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
            if d.abs() < f64::EPSILON {
                None
            } else {
                Some(l.p1 + (l.p2 - l.p1) * Point::cross(m.p2 - m.p1, m.p2 - l.p1) / d)
            }
        }

        pub fn cross_points_as_segment(l: Self, m: Self) -> Option<Point> {
            let p = Self::cross_points(l, m);
            match p {
                Some(p) => {
                    if (p - l.p1).abs() + (l.p2 - p).abs() - (l.p2 - l.p1).abs() < f64::EPSILON
                        && (p - m.p1).abs() + (m.p2 - p).abs() - (m.p2 - m.p1).abs() < f64::EPSILON
                    {
                        Some(p)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }

        /// xを与えたときのyの値を求める
        pub fn y(self, x: f64) -> Option<f64> {
            if (self.p1.x - self.p2.x).abs() < f64::EPSILON {
                None
            } else {
                Some(
                    self.p1.y + (self.p2.y - self.p1.y) / (self.p2.x - self.p1.x) * (x - self.p1.x),
                )
            }
        }

        /// yを与えたときのxの値を求める
        pub fn x(self, y: f64) -> Option<f64> {
            if (self.p1.y - self.p2.y).abs() < f64::EPSILON {
                None
            } else {
                Some(
                    self.p1.x + (self.p2.x - self.p1.x) / (self.p2.y - self.p1.y) * (y - self.p1.y),
                )
            }
        }

        pub fn distance(self, p: Point) -> f64 {
            if self.p1.x == self.p2.x {
                return (p.x - self.p1.x).abs();
            }
            if self.p1.y == self.p2.y {
                return (p.y - self.p1.y).abs();
            }
            let l = Line::new(p, p + (self.p2 - self.p1).rot90());
            match Self::cross_points(self, l) {
                Some(cp) => (p - cp).abs(),
                None => 0.0,
            }
        }
    }

    impl fmt::Display for Line {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} - {}", self.p1, self.p2)
        }
    }
}

use nalgebra::{partial_max, partial_min};
pub use reader::*;
#[allow(unused_imports)]
use {
    itertools::Itertools,
    num::Integer,
    std::{cmp::*, collections::*, io::*, num::*, str::*},
};

#[allow(dead_code)]
#[rustfmt::skip]
pub mod reader {
    #[allow(unused_imports)]
    use itertools::Itertools;
    use std::{fmt::Debug, io::*, str::*};

    pub struct Reader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        pos: usize,
    }  macro_rules! prim_method { ($name:ident: $T: ty) => { pub fn $name(&mut self) -> $T { self.n::<$T>() } }; ($name:ident) => { prim_method!($name: $name); }; } macro_rules! prim_methods { ($name:ident: $T:ty; $($rest:tt)*) => { prim_method!($name:$T); prim_methods!($($rest)*); }; ($name:ident; $($rest:tt)*) => { prim_method!($name); prim_methods!($($rest)*); }; () => () }  macro_rules! replace_expr { ($_t:tt $sub:expr) => { $sub }; } macro_rules! tuple_method { ($name: ident: ($($T:ident),+)) => { pub fn $name(&mut self) -> ($($T),+) { ($(replace_expr!($T self.n())),+) } } } macro_rules! tuple_methods { ($name:ident: ($($T:ident),+); $($rest:tt)*) => { tuple_method!($name:($($T),+)); tuple_methods!($($rest)*); }; () => () } macro_rules! vec_method { ($name: ident: ($($T:ty),+)) => { pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> { (0..n).map(|_|($(replace_expr!($T self.n())),+)).collect_vec() } }; ($name: ident: $T:ty) => { pub fn $name(&mut self, n: usize) -> Vec<$T> { (0..n).map(|_|self.n()).collect_vec() } }; } macro_rules! vec_methods { ($name:ident: ($($T:ty),+); $($rest:tt)*) => { vec_method!($name:($($T),+)); vec_methods!($($rest)*); }; ($name:ident: $T:ty; $($rest:tt)*) => { vec_method!($name:$T); vec_methods!($($rest)*); }; () => () } impl<R: BufRead> Reader<R> {
    pub fn new(reader: R) -> Reader<R> {
        let (buf, pos) = (Vec::new(), 0);
        Reader { reader, buf, pos }
    } prim_methods! { u: usize; i: i64; f: f64; str: String; c: char; string: String; u8; u16; u32; u64; u128; usize; i8; i16; i32; i64; i128; isize; f32; f64; char; } tuple_methods! { u2: (usize, usize); u3: (usize, usize, usize); u4: (usize, usize, usize, usize); i2: (i64, i64); i3: (i64, i64, i64); i4: (i64, i64, i64, i64); cuu: (char, usize, usize); } vec_methods! { uv: usize; uv2: (usize, usize); uv3: (usize, usize, usize); iv: i64; iv2: (i64, i64); iv3: (i64, i64, i64); vq: (char, usize, usize); }  pub fn n<T: FromStr>(&mut self) -> T where T::Err: Debug, { self.n_op().unwrap() }
    pub fn n_op<T: FromStr>(&mut self) -> Option<T> where T::Err: Debug, {
        if self.buf.is_empty() { self._read_next_line(); }
        let mut start = None;
        while self.pos != self.buf.len() {
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) => self.pos += 1,
                (b'\n', false) => self._read_next_line(),
                (_, false) => start = Some(self.pos),
            }
        }
        start.map(|s| from_utf8(&self.buf[s..self.pos]).unwrap().parse().unwrap())
    }
    fn _read_next_line(&mut self) {
        self.pos = 0;
        self.buf.clear();
        self.reader.read_until(b'\n', &mut self.buf).unwrap();
    }
    pub fn s(&mut self) -> Vec<char> { self.n::<String>().chars().collect() }
    pub fn char_map(&mut self, h: usize) -> Vec<Vec<char>> { (0..h).map(|_| self.s()).collect() }
    pub fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> { self.char_map(h).iter().map(|v| v.iter().map(|&c| c != ng).collect()).collect() }
    pub fn matrix(&mut self, h: usize, w: usize) -> Vec<Vec<i64>> { (0..h).map(|_| self.iv(w)).collect() }
}
}
