#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(StdinReader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: StdinReader<R>) {
    let (n, x, m) = reader.u3();
    let mut v = Vec::new();
    v.push(x);
    let mut memo = HashMap::new();
    memo.insert(x, 0);
    let mut loop_start = 0;
    let mut loop_end = 0;
    for i in 0..n {
        let next = v[i] * v[i] % m;
        if memo.contains_key(&next) {
            loop_start = *memo.get(&next).unwrap();
            loop_end = i + 1;
            break;
        }
        memo.insert(next, i + 1);
        v.push(v[i] * v[i] % m);
    }
    let mut loop_sum = 0;
    for i in loop_start..loop_end {
        loop_sum += v[i];
    }
    let mut ans = 0;

    if loop_end != 0 {
        let mut rest = n - loop_start;
        ans += rest / (loop_end - loop_start) * loop_sum;
        rest = rest % (loop_end - loop_start);

        for i in 0..loop_start + rest {
            ans += v[i];
        }
    } else {
        for i in 0..n {
            ans += v[i];
        }
    }

    println!("{}", ans);
}

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::{cmp::*, collections::*, io::*, num::*, str::*};
#[allow(unused_imports)]
use stdin_reader::StdinReader;

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
