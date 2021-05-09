#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let n = reader.u();
    let xyp = reader.iv3(n);

    let mut only_tate = HashMap::new();
    let mut only_yoko = HashMap::new();

    let p = 1 << n;
    for i in 0..p {
        let mut flag = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                flag += 1 << j;
            }
        }
        for i in 0..n {
            let mut l = xyp[i].0.abs() * xyp[i].2;
            let mut k = xyp[i].1.abs() * xyp[i].2;
            for j in 0..n {
                if flag >> j & 1 == 0 {
                    continue;
                }
                l = min(l, (xyp[j].0 - xyp[i].0).abs() * xyp[i].2);
                k = min(k, (xyp[j].1 - xyp[i].1).abs() * xyp[i].2);
            }
            only_tate.entry(flag).or_insert(vec![std::i64::MAX; n])[i] = l;
            only_yoko.entry(flag).or_insert(vec![std::i64::MAX; n])[i] = k;
        }
    }

    let mut ans = vec![std::i64::MAX; n + 1];

    let mut pattern = 1;
    for _ in 0..n {
        pattern *= 3;
    }

    //各集落に対し、縦、横、引かないの3通り
    for i in 0..pattern {
        //今回伸ばした路線の数
        let mut count = 0;

        let mut yoko = 0;
        let mut tate = 0;
        let mut i = i;
        for j in 0..n {
            let k = i % 3;
            if k == 1 {
                count += 1;
                yoko += 1 << j;
            }
            if k == 2 {
                count += 1;
                tate += 1 << j;
            }
            i /= 3;
        }

        let mut score = 0;
        let y = only_yoko.get(&yoko).unwrap();
        let t = only_tate.get(&tate).unwrap();
        for i in 0..n {
            score += min(y[i], t[i]);
        }

        ans[count] = min(ans[count], score);
    }
    for i in 0..=n {
        println!("{}", ans[i]);
    }
}

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::{cmp::*, collections::*, io::*, num::*, str::*};
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
