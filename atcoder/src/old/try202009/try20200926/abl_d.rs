#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(StdinReader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: StdinReader<R>) {
    let (n, k) = reader.u2();
    let a = reader.uv(n);

    const M: usize = 300100;
    let mut segtree = SegmentTree::new(M, 0);
    for i in 0..n {
        let left = if a[i] < k { 0 } else { a[i] - k };
        let l = segtree.get(left, min(a[i] + k + 1, M));
        segtree.update_number(a[i], l + 1);
    }

    println!("{}", segtree.get(0, M));
}

pub fn cmpfn(l: u64, r: u64) -> u64 {
    std::cmp::max(l, r)
}

struct SegmentTree {
    e: u64,
    size: usize,
    tree: Vec<u64>,
}

impl SegmentTree {
    /// 値の初期化
    pub fn new(n: usize, e: u64) -> SegmentTree {
        let mut size: usize = 1;
        while size < n {
            size <<= 1;
        }

        let mut tree: Vec<u64> = Vec::<u64>::with_capacity(2 * size);
        for _ in 0..2 * size {
            tree.push(e);
        }
        for i in (1..size).map(|x| size - x) {
            tree[i] = cmpfn(tree[2 * i], tree[2 * i + 1]);
        }

        SegmentTree { e, size, tree }
    }

    /// indexの値をnumで更新する.
    pub fn update_number(&mut self, mut index: usize, num: u64) {
        self.tree[index + self.size] = num;
        index = (index + self.size) / 2;
        while index > 0 {
            self.tree[index] = cmpfn(self.tree[2 * index], self.tree[2 * index + 1]);
            index >>= 1;
        }
    }

    /// [l, r)での範囲の答えを得る.
    pub fn get(&self, l: usize, r: usize) -> u64 {
        // println!("{}, {}, {}", self.size, l, r);
        self.rec(0, self.size, l, r)
    }

    fn rec(&self, min: usize, max: usize, l: usize, r: usize) -> u64 {
        let mid = (min + max) / 2;
        if l == min && r == max {
            return self.tree[self.size / (r - l) + l / (r - l)];
        }

        if l < mid && mid < r {
            cmpfn(self.rec(min, mid, l, mid), self.rec(mid, max, mid, r))
        } else if mid <= l {
            self.rec(mid, max, l, r)
        } else if r <= mid {
            self.rec(min, mid, l, r)
        } else {
            0
        }
    }

    fn debug(&self) {
        println!("{:?}", self.tree);
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
