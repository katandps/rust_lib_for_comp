#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(StdinReader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: StdinReader<R>) {
    let (h, w) = reader.u2();
    let s = reader.cmap(h);

    let mut yoko = vec![vec![0; w + 1]; h + 1];
    let mut tate = vec![vec![0; h + 1]; w + 1];

    // 横
    for i in 0..h {
        let mut length = 0;
        for j in 0..w {
            if s[i][j] == '.' {
                length += 1;
            } else {
                for k in j - length + 1..=j {
                    yoko[i + 1][k] += length;
                }
                length = 0;
            }
        }
        for k in w - length + 1..=w {
            yoko[i + 1][k] += length;
        }
    }
    for i in 0..w {
        let mut length = 0;
        for j in 0..h {
            if s[j][i] == '.' {
                length += 1;
            } else {
                for k in j - length + 1..=j {
                    tate[i + 1][k] += length;
                }
                length = 0;
            }
        }
        for k in h - length + 1..=h {
            tate[i + 1][k] += length;
        }
    }

    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                cnt += 1;
            }
        }
    }

    let mut ans = ModInt::new(0);
    for i in 1..=h {
        for j in 1..=w {
            if s[i - 1][j - 1] == '#' {
                continue;
            }
            let renketu = yoko[i][j] + tate[j][i] - 1;
            ans += ModInt::new(2).pow(cnt - renketu) * (ModInt::new(2).pow(renketu) - 1);
        }
    }
    println!("{}", ans);
}

#[allow(unused_imports)]
use mod_int::*;

#[allow(dead_code)]
pub mod mod_int {
    use std::fmt;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;

    const MOD: Num = 1_000_000_007;

    #[derive(Copy, Clone)]
    pub struct ModInt<T: Clone + Copy> {
        pub v: T,
    }

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            let mut t = rhs + self.v;
            if t >= MOD {
                t = t - MOD;
            }
            ModInt::new(t)
        }
    }

    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.v
        }
    }

    impl AddAssign<Num> for ModInt<Num> {
        fn add_assign(&mut self, other: Num) {
            *self = *self + other
        }
    }

    impl AddAssign<ModInt<Num>> for ModInt<Num> {
        fn add_assign(&mut self, other: ModInt<Num>) {
            *self = *self + other
        }
    }

    impl Sub<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: Num) -> ModInt<Num> {
            let rhs = if rhs >= MOD { rhs % MOD } else { rhs };
            let value = if self.v < rhs { self.v + MOD } else { self.v };
            ModInt::new(value - rhs)
        }
    }

    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.v
        }
    }

    impl SubAssign<Num> for ModInt<Num> {
        fn sub_assign(&mut self, other: Num) {
            *self = *self - other
        }
    }

    impl SubAssign<ModInt<Num>> for ModInt<Num> {
        fn sub_assign(&mut self, other: ModInt<Num>) {
            *self = *self - other
        }
    }

    impl Mul<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            ModInt::new((self.v * rhs) % MOD)
        }
    }

    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.v
        }
    }

    impl MulAssign<Num> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: Num) {
            *self = *self * rhs
        }
    }

    impl MulAssign<ModInt<Num>> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self * rhs
        }
    }

    impl Div<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            self * ModInt::new(rhs).pow(MOD - 2)
        }
    }

    impl Div<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self / rhs.v
        }
    }

    impl DivAssign<Num> for ModInt<Num> {
        fn div_assign(&mut self, rhs: Num) {
            *self = *self / rhs
        }
    }

    impl DivAssign<ModInt<Num>> for ModInt<Num> {
        fn div_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self / rhs
        }
    }

    impl fmt::Display for ModInt<Num> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.v)
        }
    }

    impl fmt::Debug for ModInt<Num> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.v)
        }
    }

    impl ModInt<Num> {
        pub fn pow(self, e: Num) -> ModInt<Num> {
            let mut result = ModInt::new(1);
            let mut cur = self;
            let mut e = e;
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                }
                e >>= 1;
                cur *= cur;
            }
            result
        }

        pub fn new(v: Num) -> ModInt<Num> {
            let v = if v >= MOD { v % MOD } else { v };
            ModInt { v }
        }

        pub fn get(&self) -> Num {
            self.v
        }
    }
}

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::{cmp::*, collections::*, io::*, num::*, str::*};
#[allow(unused_imports)]
pub use stdin_reader::StdinReader;

#[allow(dead_code)]
pub mod stdin_reader {
    use std::{fmt::Debug, io::*, str::*};

    pub struct StdinReader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        // Should never be empty
        pos: usize, // Should never be out of bounds as long as the input ends with '\n'
    }

    impl<R: BufRead> StdinReader<R> {
        pub fn new(reader: R) -> StdinReader<R> {
            let (buf, pos) = (Vec::new(), 0);
            StdinReader { reader, buf, pos }
        }

        pub fn n<T: FromStr>(&mut self) -> T
        where
            T::Err: Debug,
        {
            if self.buf.is_empty() {
                self._read_next_line();
            }
            let mut start = None;
            while self.pos != self.buf.len() {
                match (self.buf[self.pos], start.is_some()) {
                    (b' ', true) | (b'\n', true) => break,
                    (_, true) | (b' ', false) => self.pos += 1,
                    (b'\n', false) => self._read_next_line(),
                    (_, false) => start = Some(self.pos),
                }
            }
            match start {
                Some(s) => from_utf8(&self.buf[s..self.pos]).unwrap().parse().unwrap(),
                None => panic!("入力された数を超えた読み込みが発生しています"),
            }
        }

        fn _read_next_line(&mut self) {
            self.pos = 0;
            self.buf.clear();
            if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
                panic!("Reached EOF");
            }
        }

        pub fn str(&mut self) -> String {
            self.n()
        }
        pub fn s(&mut self) -> Vec<char> {
            self.n::<String>().chars().collect()
        }
        pub fn i(&mut self) -> i64 {
            self.n()
        }
        pub fn i2(&mut self) -> (i64, i64) {
            (self.n(), self.n())
        }
        pub fn i3(&mut self) -> (i64, i64, i64) {
            (self.n(), self.n(), self.n())
        }
        pub fn i4(&mut self) -> (i64, i64, i64, i64) {
            (self.n(), self.n(), self.n(), self.n())
        }
        pub fn u(&mut self) -> usize {
            self.n()
        }
        pub fn u2(&mut self) -> (usize, usize) {
            (self.n(), self.n())
        }
        pub fn u3(&mut self) -> (usize, usize, usize) {
            (self.n(), self.n(), self.n())
        }
        pub fn u4(&mut self) -> (usize, usize, usize, usize) {
            (self.n(), self.n(), self.n(), self.n())
        }
        pub fn u5(&mut self) -> (usize, usize, usize, usize, usize) {
            (self.n(), self.n(), self.n(), self.n(), self.n())
        }
        pub fn u6(&mut self) -> (usize, usize, usize, usize, usize, usize) {
            (self.n(), self.n(), self.n(), self.n(), self.n(), self.n())
        }
        pub fn f(&mut self) -> f64 {
            self.n()
        }
        pub fn f2(&mut self) -> (f64, f64) {
            (self.n(), self.n())
        }
        pub fn c(&mut self) -> char {
            self.n::<String>().pop().unwrap()
        }
        pub fn iv(&mut self, n: usize) -> Vec<i64> {
            (0..n).map(|_| self.i()).collect()
        }
        pub fn iv2(&mut self, n: usize) -> Vec<(i64, i64)> {
            (0..n).map(|_| self.i2()).collect()
        }
        pub fn iv3(&mut self, n: usize) -> Vec<(i64, i64, i64)> {
            (0..n).map(|_| self.i3()).collect()
        }
        pub fn uv(&mut self, n: usize) -> Vec<usize> {
            (0..n).map(|_| self.u()).collect()
        }
        pub fn uv2(&mut self, n: usize) -> Vec<(usize, usize)> {
            (0..n).map(|_| self.u2()).collect()
        }
        pub fn uv3(&mut self, n: usize) -> Vec<(usize, usize, usize)> {
            (0..n).map(|_| self.u3()).collect()
        }
        pub fn uv4(&mut self, n: usize) -> Vec<(usize, usize, usize, usize)> {
            (0..n).map(|_| self.u4()).collect()
        }
        pub fn fv(&mut self, n: usize) -> Vec<f64> {
            (0..n).map(|_| self.f()).collect()
        }
        pub fn cmap(&mut self, h: usize) -> Vec<Vec<char>> {
            (0..h).map(|_| self.s()).collect()
        }
    }
}
