#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(StdinReader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: StdinReader<R>) {
    let n = reader.u();

    let mut d = Vec::new();
    let mut l = 1;
    let k = n * 2;
    while l * l <= k {
        if k % l == 0 {
            d.push(l);
            d.push(k / l);
        }
        l += 1;
    }

    let mut ans = 2 * n;
    for div in d {
        let other = k / div;
        if other == 1 {
            continue;
        }
        let t = crt(0, div, other - 1, other).0;
        if t != 0 {
            ans = min(ans, t);
        }
    }
    println!("{}", ans);
}

#[allow(unused_imports)]
use chinese_remainder_theorem::*;

#[allow(dead_code)]
mod chinese_remainder_theorem {
    /// 中国剰余定理
    /// x === b1 mod m, x === b2 mod m となる整数xを返す(x === r mod m)
    /// (r, m) の順で返却
    /// 値がない場合は(0,0)を返す
    pub fn crt(b1: usize, m1: usize, b2: usize, m2: usize) -> (usize, usize) {
        let (b1, b2, m1, m2) = (b1 as i128, b2 as i128, m1 as i128, m2 as i128);
        let (d, p, _q) = ext_gcd(m1, m2);
        if (b2 - b1).abs() % d != 0 {
            (0, 0)
        } else {
            let m = m1 * (m2 / d); //lcm
            let tmp = (b2 - b1).abs() / d * p % (m2 / d);
            let r = mo(b1 + m1 * tmp, m);
            (r as usize, m as usize)
        }
    }

    ///拡張Euclidの互除法 返り値 (d, p, q)
    /// (p,q) は ap + bq = gcd(a, b) となるp, q
    /// d は gcd(a,b)
    fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (d, q, p) = ext_gcd(b, a % b);
            let q = q - (a / b) * p;
            (d, p, q)
        }
    }

    fn mo(a: i128, m: i128) -> i128 {
        (a % m + m) % m
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
