#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let n = reader.u();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut u = Vec::new();
    for _ in 0..n {
        x.push(reader.i());
        y.push(reader.i());
        u.push(reader.c());
    }
    let mut ans = std::i64::MAX;
    //case1 同一直線状に飛行機がいて、向かい合って衝突する
    // ぶつかるまでの時間は5(Bigger_x - Smaller_x)

    // U,Dの組 xでマッピング
    let mut map = HashMap::new();
    for i in 0..n {
        map.entry(x[i])
            .or_insert(HashMap::new())
            .entry(u[i])
            .or_insert(Vec::new())
            .push(y[i]);
    }

    for (_, map) in map {
        if map.contains_key(&'D') && map.contains_key(&'U') {
            let k = mm(
                map.get(&'D').unwrap().clone(),
                map.get(&'U').unwrap().clone(),
            );
            if k != std::i64::MAX {
                ans = min(ans, k / 2);
            }
        }
    }

    // L,Rの組 yでマッピング
    let mut map = HashMap::new();
    for i in 0..n {
        map.entry(y[i])
            .or_insert(HashMap::new())
            .entry(u[i])
            .or_insert(Vec::new())
            .push(x[i]);
    }

    for (_, map) in map {
        if map.contains_key(&'L') && map.contains_key(&'R') {
            let k = mm(
                map.get(&'L').unwrap().clone(),
                map.get(&'R').unwrap().clone(),
            );
            if k != std::i64::MAX {
                ans = min(ans, k / 2);
            }
        }
    }

    //case2 垂直をなす飛行機が同時に同じ座標に到達する
    // RとU、LとDの組
    // x+yが同じ組について衝突する可能性があるので、分類する
    // ぶつかるまでの時間は RとUの組が 10(Ux-Rx) LとDの組が 10(Lx-Dx)
    //HashMap<x+y, HashMap<|L,D,U,R|, Vec<x>>>

    let mut map = HashMap::new();
    for i in 0..n {
        map.entry(x[i] + y[i])
            .or_insert(HashMap::new())
            .entry(u[i])
            .or_insert(Vec::new())
            .push(x[i]);
    }
    for (_, map) in map {
        if map.contains_key(&'R') && map.contains_key(&'U') {
            ans = min(
                ans,
                mm(
                    map.get(&'U').unwrap().clone(),
                    map.get(&'R').unwrap().clone(),
                ),
            );
        }
        if map.contains_key(&'L') && map.contains_key(&'D') {
            ans = min(
                ans,
                mm(
                    map.get(&'L').unwrap().clone(),
                    map.get(&'D').unwrap().clone(),
                ),
            );
        }
    }

    // RとD、LとUの組
    // x-yが同じ組について衝突する可能性があるので、分類する
    // ぶつかるまでの時間は RとDの組が 10(Dx-Rx) LとUの組が 10(Lx-Ux)
    //HashMap<x-y, HashMap<|L,D,U,R|, Vec<x>>>

    let mut map = HashMap::new();
    for i in 0..n {
        map.entry(x[i] - y[i])
            .or_insert(HashMap::new())
            .entry(u[i])
            .or_insert(Vec::new())
            .push(x[i]);
    }

    for (_, map) in map {
        if map.contains_key(&'R') && map.contains_key(&'D') {
            ans = min(
                ans,
                mm(
                    map.get(&'D').unwrap().clone(),
                    map.get(&'R').unwrap().clone(),
                ),
            );
        }
        if map.contains_key(&'L') && map.contains_key(&'U') {
            ans = min(
                ans,
                mm(
                    map.get(&'L').unwrap().clone(),
                    map.get(&'U').unwrap().clone(),
                ),
            );
        }
    }

    if ans == std::i64::MAX {
        println!("SAFE");
    } else {
        println!("{}", ans);
    }
}

fn mm(mut v1: Vec<i64>, mut v2: Vec<i64>) -> i64 {
    v1.sort();
    v2.sort();
    let mut ret = std::i64::MAX;
    let mut big = v1[0];
    let mut small = v2[0];
    let mut big_i = 1;
    let mut small_i = 1;
    loop {
        while big < small && big_i < v1.len() {
            big = v1[big_i];
            big_i += 1;
        }
        while small_i < v2.len() && v2[small_i] < big {
            small = v2[small_i];
            small_i += 1;
        }
        if big > small {
            ret = min(ret, big - small);
        }
        big_i += 1;
        if big_i >= v1.len() {
            break;
        }
    }
    if ret == std::i64::MAX {
        ret
    } else {
        ret * 10
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
