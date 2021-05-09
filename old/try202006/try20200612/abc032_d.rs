#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let (n, w) = reader.u2();
    let vw = reader.uv2(n);

    let mut w_max = 0;
    let mut v_max = 0;
    for &(v, w) in &vw {
        w_max = max(w_max, w);
        v_max = max(v_max, v);
    }
    // 重さが小さい
    if w_max <= 1000 {
        // dp[i][w] = v i番目まで選んだときに重さがwになる組み合わせの価値の最大値はv
        let mut dp = vec![vec![0; 200001]; n + 1];
        for i in 1..n + 1 {
            let (v, w) = vw[i - 1];
            for j in 0..200001 {
                dp[i][j] = max(dp[i][j], dp[i - 1][j]);
                if w + j > 200000 {
                    continue;
                }
                dp[i][w + j] = max(dp[i][w + j], dp[i - 1][j] + v);
            }
        }
        let mut ans = 0;
        for k in 0..w + 1 {
            ans = max(ans, dp[n][k]);
        }
        println!("{}", ans);

        return;
    }
    // 価値が小さい
    if v_max <= 1000 {
        // dp[i][v] = w i番目まで選んだときに価値がvになる組み合わせの重さの最小値はw
        let mut dp = vec![vec![std::usize::MAX; 200001]; n + 1];
        dp[0][0] = 0usize;
        for i in 1..n + 1 {
            let (v, w) = vw[i - 1];
            for j in 0..200001 {
                dp[i][j] = min(dp[i][j], dp[i - 1][j]);
                if v + j > 200000 {
                    continue;
                }
                if dp[i - 1][j] == std::usize::MAX {
                    continue;
                }
                dp[i][v + j] = min(dp[i][v + j], dp[i - 1][j] + w);
            }
        }
        let mut ans = 0;
        for i in 0..200001 {
            if dp[n][i] <= w {
                ans = max(ans, i);
            }
        }
        println!("{}", ans);
        return;
    }

    // nが小さい
    // 未実装のため、部分点66点
}

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
