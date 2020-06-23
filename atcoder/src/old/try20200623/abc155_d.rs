#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let (n, k) = reader.u2();
    let a = reader.iv(n);
    let ms = a
        .iter()
        .filter(|&&x| x < 0)
        .cloned()
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    let ps = a
        .iter()
        .filter(|&&x| x > 0)
        .cloned()
        .sorted()
        .collect::<Vec<_>>();
    let plus = ps.len();
    let minus = ms.len();
    let zero = n - plus - minus;
    let res_minus = plus * minus;
    let res_zero = if zero > 0 {
        zero * (plus + minus) + zero * (zero - 1) / 2
    } else {
        0
    };

    //    dbg!(res_minus, res_zero, res_plus);
    if k <= res_minus {
        // k番目の値を探す
        let k = res_minus - k + 1;
        //1から10^18までの数のうち、積がこの値以下になるものがk個未満になる最大の値を二分探索で探す
        let mut ng: i64 = -1;
        let mut ok = 1_000_000_000_000_000_001;

        let count = |ms: &Vec<i64>, ps: &Vec<i64>, m: i64| -> usize {
            let mut count = 0;
            for &m_i in ms {
                let mut ng = -1;
                let mut ok = ps.len() as i32;
                while (ok - ng).abs() > 1 {
                    let mid = (ok + ng) / 2;
                    if -ps[mid as usize] * m_i >= m {
                        ok = mid
                    } else {
                        ng = mid
                    }
                }
                count += ok as usize;
            }
            //dbg!(&m, &count);
            count
        };
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if count(&ms, &ps, mid) >= k {
                ok = mid
            } else {
                ng = mid
            }
        }
        println!("{}", -ok + 1);
        // dbg!(ms, ps, k);
        return;
    } else if k <= res_minus + res_zero {
        println!("{}", 0);
        return;
    }
    let k = k - res_minus - res_zero;
    //1から10^18までの数のうち、積がこの値以下になるものがk個未満になる最大の値を二分探索で探す
    let mut ng: i64 = -1;
    let mut ok = 1_000_000_000_000_000_001;

    let count = |ms: &Vec<i64>, ps: &Vec<i64>, m: i64| -> usize {
        let mut count = 0;
        if ms.len() > 0 {
            for i in 0..ms.len() - 1 {
                let mut ng = i;
                let mut ok = ms.len();
                while ok - ng > 1 {
                    let mid = (ok + ng) / 2;
                    if ms[mid] * ms[i] > m {
                        ok = mid
                    } else {
                        ng = mid
                    }
                }
                count += ok - i - 1;
            }
        }
        if ps.len() > 0 {
            for i in 0..ps.len() - 1 {
                let mut ng = i;
                let mut ok = ps.len();
                while ok - ng > 1 {
                    let mid = (ok + ng) / 2;
                    if ps[mid] * ps[i] > m {
                        ok = mid
                    } else {
                        ng = mid
                    }
                }
                count += ok - i - 1;
                // dbg!(ok, i);
            }
        }
        // dbg!(&m, &count);
        count
    };

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if count(&ms, &ps, mid) >= k {
            ok = mid
        } else {
            ng = mid
        }
    }
    println!("{}", ok);
    // dbg!(ps, ms, k);
}

use itertools::Itertools;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::num::*;
#[allow(unused_imports)]
use std::str::*;
#[allow(unused_imports)]
use stdin_reader::StdinReader;

#[allow(dead_code)]
mod stdin_reader {
    use std::fmt::Debug;
    use std::io::*;
    use std::str::*;

    pub struct StdinReader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        // Should never be empty
        pos: usize, // Should never be out of bounds as long as the input ends with '\n'
    }

    impl<R: BufRead> StdinReader<R> {
        pub fn new(r: R) -> StdinReader<R> {
            StdinReader {
                reader: r,
                buf: Vec::new(),
                pos: 0,
            }
        }
        pub fn next<T: FromStr>(&mut self) -> T
        where
            T::Err: Debug,
        {
            if self.buf.is_empty() {
                self._read_next_line();
            }
            let mut start = None;
            loop {
                if self.pos == self.buf.len() {
                    break;
                }
                match (self.buf[self.pos], start.is_some()) {
                    (b' ', true) | (b'\n', true) => break,
                    (_, true) | (b' ', false) => self.pos += 1,
                    (b'\n', false) => self._read_next_line(),
                    (_, false) => start = Some(self.pos),
                }
            }
            let target = &self.buf[start.unwrap()..self.pos];
            from_utf8(target).unwrap().parse().unwrap()
        }

        fn _read_next_line(&mut self) {
            self.pos = 0;
            self.buf.clear();
            if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
                panic!("Reached EOF");
            }
        }

        pub fn str(&mut self) -> String {
            self.next()
        }
        pub fn s(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }
        pub fn i(&mut self) -> i64 {
            self.next()
        }
        pub fn i2(&mut self) -> (i64, i64) {
            (self.next(), self.next())
        }
        pub fn i3(&mut self) -> (i64, i64, i64) {
            (self.next(), self.next(), self.next())
        }
        pub fn u(&mut self) -> usize {
            self.next()
        }
        pub fn u2(&mut self) -> (usize, usize) {
            (self.next(), self.next())
        }
        pub fn u3(&mut self) -> (usize, usize, usize) {
            (self.next(), self.next(), self.next())
        }
        pub fn u4(&mut self) -> (usize, usize, usize, usize) {
            (self.next(), self.next(), self.next(), self.next())
        }
        pub fn u5(&mut self) -> (usize, usize, usize, usize, usize) {
            (
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
            )
        }
        pub fn u6(&mut self) -> (usize, usize, usize, usize, usize, usize) {
            (
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
            )
        }
        pub fn f(&mut self) -> f64 {
            self.next()
        }
        pub fn f2(&mut self) -> (f64, f64) {
            (self.next(), self.next())
        }
        pub fn c(&mut self) -> char {
            self.next::<String>().pop().unwrap()
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
