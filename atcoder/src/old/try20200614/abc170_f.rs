#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let (h, w, k) = reader.u3();
    let (x1, y1, x2, y2) = reader.u4();
    let map = reader.cmap(h);
    let (x1, y1, x2, y2) = (y1 - 1, x1 - 1, y2 - 1, x2 - 1);
    let mut grid = Grid::new(h, w, map);
    let mut dist = vec![std::usize::MAX; h * w];
    let mut q = VecDeque::new();
    q.push_front(grid.key(x1, y1));
    dist[grid.key(x1, y1)] = 0;
    while q.len() > 0 {
        let cur = q.pop_front().unwrap();
        // dbg!(
        //     &cur,
        //     &dist,
        //     &grid.neighbor(cur, k, dist[cur] + 1, &dist, '@')
        // );
        for neigh in grid.neighbor(cur, k, dist[cur] + 1, &dist, '@') {
            if dist[cur] + 1 < dist[neigh] {
                dist[neigh] = dist[cur] + 1;
                q.push_back(neigh);
            }
        }
    }

    if dist[grid.key(x2, y2)] == std::usize::MAX {
        println!("{}", -1);
    } else {
        println!("{}", dist[grid.key(x2, y2)]);
    }
}

#[allow(unused_imports)]
use grid::*;

#[allow(dead_code)]
mod grid {
    #[derive(Debug)]
    pub struct Grid<T: Eq> {
        pub h: usize,
        pub w: usize,
        pub max: usize,
        pub map: Vec<T>,
    }

    impl<T: Clone + Eq> Grid<T> {
        pub fn new(h: usize, w: usize, map: Vec<Vec<T>>) -> Grid<T> {
            let mut flat = Vec::new();
            for r in map {
                for c in r {
                    flat.push(c);
                }
            }
            Grid {
                h: h,
                w: w,
                max: h * w,
                map: flat,
            }
        }
        pub fn key(&self, x: usize, y: usize) -> usize {
            y * self.w + x
        }
        pub fn xy(&self, k: usize) -> (usize, usize) {
            (self.x(k), self.y(k))
        }
        pub fn x(&self, k: usize) -> usize {
            k % self.w
        }
        pub fn y(&self, k: usize) -> usize {
            k / self.w
        }
        pub fn get(&self, key: usize) -> &T {
            &self.map[key]
        }
        pub fn set(&mut self, key: usize, value: T) {
            self.map[key] = value;
        }
        pub fn neighbor(
            &self,
            key: usize,
            k: usize,
            d: usize,
            dist: &Vec<usize>,
            ng: T,
        ) -> Vec<usize> {
            let mut ret = Vec::new();
            let mut l = self.x(key);
            while self.x(key) >= l {
                if self.map[key - l] == ng {
                    break;
                }
                if dist[key - l] > d {
                    ret.push(key - l)
                }
                l += 1;
                if l > k {
                    break;
                }
            }
            l = 1;
            while self.y(key) >= l {
                if self.map[key - l * self.w] == ng {
                    break;
                }
                if dist[key - l * self.w] > d {
                    ret.push(key - l * self.w);
                }
                l += 1;
                if l > k {
                    break;
                }
            }
            l = 1;
            while self.x(key) + l < self.w {
                if self.map[key + l] == ng {
                    break;
                }
                if dist[key + l] > d {
                    ret.push(key + l);
                }
                l += 1;
                if l > k {
                    break;
                }
            }
            l = 1;
            while self.y(key) + l < self.h {
                if self.map[key + l * self.w] == ng {
                    break;
                }
                if dist[key + self.w * l] > d {
                    ret.push(key + self.w * l)
                }
                l += 1;
                if l > k {
                    break;
                }
            }
            ret
        }
    }
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
