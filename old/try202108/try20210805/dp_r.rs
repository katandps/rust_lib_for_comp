pub use reader::*;
#[allow(unused_imports)]
use {
    itertools::Itertools,
    num::Integer,
    std::convert::TryInto,
    std::{cmp::*, collections::*, io::*, num::*, str::*},
};

#[allow(unused_macros)]
macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
#[allow(unused_macros)]
macro_rules! chmax {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_max = max!($($cmps),+);if $base < cmp_max {$base = cmp_max;true} else {false}}};}
#[allow(unused_macros)]
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}
#[allow(unused_macros)]
macro_rules! max {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$a} else {$b}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = max!($($rest),+);if $a > b {$a} else {b}}};
}

#[allow(dead_code)]
#[rustfmt::skip]
pub mod reader { #[allow(unused_imports)]use itertools::Itertools;use std::{fmt::Debug, io::*, str::*};pub struct Reader<R: BufRead> {reader: R,buf: Vec<u8>,pos: usize,}  macro_rules! prim_method { ($name:ident: $T: ty) => { pub fn $name(&mut self) -> $T { self.n::<$T>() } }; ($name:ident) => { prim_method!($name: $name); }; } macro_rules! prim_methods { ($name:ident: $T:ty; $($rest:tt)*) => { prim_method!($name:$T); prim_methods!($($rest)*); }; ($name:ident; $($rest:tt)*) => { prim_method!($name); prim_methods!($($rest)*); }; () => () }  macro_rules! replace_expr { ($_t:tt $sub:expr) => { $sub }; } macro_rules! tuple_method { ($name: ident: ($($T:ident),+)) => { pub fn $name(&mut self) -> ($($T),+) { ($(replace_expr!($T self.n())),+) } } } macro_rules! tuple_methods { ($name:ident: ($($T:ident),+); $($rest:tt)*) => { tuple_method!($name:($($T),+)); tuple_methods!($($rest)*); }; () => () } macro_rules! vec_method { ($name: ident: ($($T:ty),+)) => { pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> { (0..n).map(|_|($(replace_expr!($T self.n())),+)).collect_vec() } }; ($name: ident: $T:ty) => { pub fn $name(&mut self, n: usize) -> Vec<$T> { (0..n).map(|_|self.n()).collect_vec() } }; } macro_rules! vec_methods { ($name:ident: ($($T:ty),+); $($rest:tt)*) => { vec_method!($name:($($T),+)); vec_methods!($($rest)*); }; ($name:ident: $T:ty; $($rest:tt)*) => { vec_method!($name:$T); vec_methods!($($rest)*); }; () => () } impl<R: BufRead> Reader<R> {pub fn new(reader: R) -> Reader<R> {let (buf, pos) = (Vec::new(), 0);Reader { reader, buf, pos }} prim_methods! { u: usize; i: i64; f: f64; str: String; c: char; string: String; u8; u16; u32; u64; u128; usize; i8; i16; i32; i64; i128; isize; f32; f64; char; } tuple_methods! { u2: (usize, usize); u3: (usize, usize, usize); u4: (usize, usize, usize, usize); i2: (i64, i64); i3: (i64, i64, i64); i4: (i64, i64, i64, i64); cuu: (char, usize, usize); } vec_methods! { uv: usize; uv2: (usize, usize); uv3: (usize, usize, usize); iv: i64; iv2: (i64, i64); iv3: (i64, i64, i64); vq: (char, usize, usize); }  pub fn n<T: FromStr>(&mut self) -> T where T::Err: Debug, { self.n_op().unwrap() }pub fn n_op<T: FromStr>(&mut self) -> Option<T> where T::Err: Debug, {if self.buf.is_empty() { self._read_next_line(); }let mut start = None;while self.pos != self.buf.len() {match (self.buf[self.pos], start.is_some()) {(b' ', true) | (b'\n', true) => break,(_, true) | (b' ', false) => self.pos += 1,(b'\n', false) => self._read_next_line(),(_, false) => start = Some(self.pos),}}start.map(|s| from_utf8(&self.buf[s..self.pos]).unwrap().parse().unwrap())}fn _read_next_line(&mut self) {self.pos = 0;self.buf.clear();self.reader.read_until(b'\n', &mut self.buf).unwrap();}pub fn s(&mut self) -> Vec<char> { self.n::<String>().chars().collect() }pub fn char_map(&mut self, h: usize) -> Vec<Vec<char>> { (0..h).map(|_| self.s()).collect() }pub fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> { self.char_map(h).iter().map(|v| v.iter().map(|&c| c != ng).collect()).collect() }pub fn matrix(&mut self, h: usize, w: usize) -> Vec<Vec<i64>> { (0..h).map(|_| self.iv(w)).collect() }}}

#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (n, k) = (reader.u(), reader.i());
    let a = reader.matrix(n, n);
    let a = a
        .into_iter()
        .map(|v| v.into_iter().map(|ai| Mi::new(ai)).collect_vec())
        .collect_vec();

    let mat: Matrix = a.try_into().unwrap();
    let res = mat.pow(k).unwrap();
    let mut ans = Mi::new(0);
    for i in 0..n {
        for j in 0..n {
            ans += res.buf[j][i];
        }
    }
    println!("{}", ans);
}

#[allow(unused_imports)]
use mod_int::*;

#[allow(dead_code)]
pub mod mod_int {
    use std::marker::PhantomData;
    use std::ops::*;

    pub trait Mod: Copy + Clone + std::fmt::Debug {
        fn get() -> i64;
    }

    pub type Mi = ModInt<Mod1e9p7>;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Mod1e9p7;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Mod1e9p9;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Mod998244353;

    impl Mod for Mod1e9p7 {
        fn get() -> i64 {
            1_000_000_007
        }
    }

    impl Mod for Mod1e9p9 {
        fn get() -> i64 {
            1_000_000_009
        }
    }

    impl Mod for Mod998244353 {
        fn get() -> i64 {
            998_244_353
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModInt<M: Mod> {
        n: i64,
        _p: PhantomData<M>,
    }

    impl<M: Mod> ModInt<M> {
        pub fn new(n: i64) -> Self {
            Self {
                n: n.rem_euclid(M::get()),
                _p: PhantomData,
            }
        }

        pub fn pow(mut self, mut e: i64) -> ModInt<M> {
            let mut result = Self::new(1);
            while e > 0 {
                if e & 1 == 1 {
                    result *= self.n;
                }
                e >>= 1;
                self *= self.n;
            }
            result
        }

        pub fn get(&self) -> i64 {
            self.n
        }
    }

    impl<M: Mod> Add<i64> for ModInt<M> {
        type Output = Self;
        fn add(self, rhs: i64) -> Self {
            ModInt::new(self.n + rhs.rem_euclid(M::get()))
        }
    }

    impl<M: Mod> Add<ModInt<M>> for ModInt<M> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            self + rhs.n
        }
    }

    impl<M: Mod> AddAssign<i64> for ModInt<M> {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs
        }
    }

    impl<M: Mod> AddAssign<ModInt<M>> for ModInt<M> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self {
            Self::new(-self.n)
        }
    }

    impl<M: Mod> Sub<i64> for ModInt<M> {
        type Output = Self;
        fn sub(self, rhs: i64) -> Self {
            ModInt::new(self.n - rhs.rem_euclid(M::get()))
        }
    }

    impl<M: Mod> Sub<ModInt<M>> for ModInt<M> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            self - rhs.n
        }
    }

    impl<M: Mod> SubAssign<i64> for ModInt<M> {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs
        }
    }

    impl<M: Mod> SubAssign<ModInt<M>> for ModInt<M> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl<M: Mod> Mul<i64> for ModInt<M> {
        type Output = Self;
        fn mul(self, rhs: i64) -> Self {
            ModInt::new(self.n * (rhs % M::get()))
        }
    }

    impl<M: Mod> Mul<ModInt<M>> for ModInt<M> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            self * rhs.n
        }
    }

    impl<M: Mod> MulAssign<i64> for ModInt<M> {
        fn mul_assign(&mut self, rhs: i64) {
            *self = *self * rhs
        }
    }

    impl<M: Mod> MulAssign<ModInt<M>> for ModInt<M> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs
        }
    }

    impl<M: Mod> Div<i64> for ModInt<M> {
        type Output = Self;
        fn div(self, rhs: i64) -> Self {
            self * ModInt::new(rhs).pow(M::get() - 2)
        }
    }

    impl<M: Mod> Div<ModInt<M>> for ModInt<M> {
        type Output = Self;
        fn div(self, rhs: Self) -> Self {
            self / rhs.n
        }
    }

    impl<M: Mod> DivAssign<i64> for ModInt<M> {
        fn div_assign(&mut self, rhs: i64) {
            *self = *self / rhs
        }
    }

    impl<M: Mod> DivAssign<ModInt<M>> for ModInt<M> {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs
        }
    }

    impl<M: Mod> std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.n)
        }
    }

    impl<M: Mod> std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.n)
        }
    }

    impl<M: Mod> Deref for ModInt<M> {
        type Target = i64;
        fn deref(&self) -> &Self::Target {
            &self.n
        }
    }

    impl<M: Mod> DerefMut for ModInt<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.n
        }
    }

    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(i: i64) -> Self {
            Self::new(i)
        }
    }

    impl<M: Mod> From<ModInt<M>> for i64 {
        fn from(m: ModInt<M>) -> Self {
            m.n
        }
    }
}

#[allow(unused_imports)]
use matrix::*;

#[allow(dead_code)]
mod matrix {
    use super::*;
    use itertools::Itertools;
    use std::convert::TryInto;
    use std::fmt;
    use std::ops;

    #[derive(Clone, Eq, PartialEq)]
    pub struct Matrix {
        pub buf: Vec<Vec<Mi>>,
    }

    impl std::convert::TryFrom<Vec<Vec<Mi>>> for Matrix {
        type Error = &'static str;
        fn try_from(buf: Vec<Vec<Mi>>) -> std::result::Result<Self, Self::Error> {
            if (1..buf.len()).any(|i| buf[0].len() != buf[i].len()) {
                Err("size is invalid")
            } else {
                Ok(Self { buf })
            }
        }
    }

    impl Matrix {
        fn identity_matrix(n: usize) -> Self {
            let mut buf = vec![vec![Mi::new(0); n]; n];
            for i in 0..n {
                buf[i][i] += 1;
            }
            Matrix { buf }
        }

        /// (y, x)
        fn size(&self) -> (usize, usize) {
            if self.buf.len() == 0 {
                (0, 0)
            } else {
                (self.buf.len(), self.buf[0].len())
            }
        }

        /// y行目、x列目を除いた 余因子行列を計算する
        /// x, y は 0-indexed
        pub fn sub_matrix(&self, x: usize, y: usize) -> Self {
            let (n, m) = self.size();
            let mut sub = vec![vec![Mi::new(0); m - 1]; n - 1];
            for yi in (0..n).filter(|&yi| yi != y) {
                for xi in (0..m).filter(|&xi| xi != x) {
                    sub[yi - if yi < y { 0 } else { 1 }][xi - if xi < x { 0 } else { 1 }] =
                        self.buf[yi][xi];
                }
            }
            Matrix { buf: sub }
        }

        /// 行列式detを計算する
        /// 平方行列でない場合はNoneを返す
        /// 計算量は O(size^3)
        pub fn determinant(&self) -> Option<Mi> {
            let (n, m) = self.size();
            let zero = Mi::new(0);
            if n != m {
                return None;
            }
            if n == 0 {
                return Some(zero);
            }

            let mut res = Mi::new(1);
            let mut buf = self.buf.clone();
            for i in 0..n {
                match (i..n).find(|&ni| buf[ni][i] != zero) {
                    Some(ni) => buf.swap(i, ni),
                    None => return Some(zero),
                }
                res *= buf[i][i];
                let diag = Mi::new(1) / buf[i][i];
                (i..n).for_each(|j| buf[i][j] *= diag);
                for ni in (0..n).filter(|&ni| ni != i) {
                    let c = buf[ni][i];
                    for j in i..n {
                        let d = c * buf[i][j];
                        buf[ni][j] -= d;
                    }
                }
            }

            Some(res)
        }

        pub fn pow(mut self, mut e: i64) -> Option<Self> {
            let (n, m) = self.size();
            if n != m {
                return None;
            }
            let mut result = Self::identity_matrix(n);
            while e > 0 {
                if e & 1 == 1 {
                    result = (result * self.clone()).unwrap();
                }
                e >>= 1;
                self = (self.clone() * self).unwrap();
            }
            Some(result)
        }
    }

    impl ops::Add<Matrix> for Matrix {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self {
            for i in 0..self.buf.len() {
                for j in 0..self.buf[0].len() {
                    self.buf[i][j] += rhs.buf[i][j]
                }
            }
            self
        }
    }

    impl ops::Neg for Matrix {
        type Output = Self;
        fn neg(mut self) -> Self {
            for i in 0..self.buf.len() {
                for j in 0..self.buf[0].len() {
                    self.buf[i][j] = -self.buf[i][j]
                }
            }
            self
        }
    }

    impl ops::Sub<Matrix> for Matrix {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            self + (-rhs)
        }
    }

    impl ops::Mul<i64> for Matrix {
        type Output = Self;
        fn mul(mut self, rhs: i64) -> Self {
            let (n, m) = self.size();
            for i in 0..n {
                for j in 0..m {
                    self.buf[i][j] *= rhs;
                }
            }
            self
        }
    }

    impl ops::Mul<Matrix> for Matrix {
        type Output = Option<Matrix>;
        fn mul(self, rhs: Matrix) -> Option<Matrix> {
            let ((self_y, self_x), (rhs_y, rhs_x)) = (self.size(), rhs.size());
            if self_x != rhs_y {
                return None;
            }
            let mut buf = vec![vec![Mi::new(0); rhs_x]; self_y];
            for i in 0..self_y {
                for j in 0..rhs_x {
                    for k in 0..self_x {
                        buf[i][j] += self.buf[i][k] * rhs.buf[k][j];
                    }
                }
            }
            Some(buf.try_into().unwrap())
        }
    }

    impl fmt::Debug for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}",
                self.buf.iter().map(|row| row.iter().join(" ")).join("\n")
            )
        }
    }
}
////////////////////////////////////////////////////////
