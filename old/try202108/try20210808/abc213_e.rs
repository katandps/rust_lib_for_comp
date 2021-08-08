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
pub mod reader { #[allow(unused_imports)] use itertools::Itertools; use std::{fmt::Debug, io::*, str::*};  pub struct Reader<R: BufRead> { reader: R, buf: Vec<u8>, pos: usize, }  macro_rules! prim_method { ($name:ident: $T: ty) => { pub fn $name(&mut self) -> $T { self.n::<$T>() } }; ($name:ident) => { prim_method!($name: $name); } } macro_rules! prim_methods { ($name:ident: $T:ty; $($rest:tt)*) => { prim_method!($name:$T); prim_methods!($($rest)*); }; ($name:ident; $($rest:tt)*) => { prim_method!($name); prim_methods!($($rest)*); }; () => () }  macro_rules! replace_expr { ($_t:tt $sub:expr) => { $sub }; } macro_rules! tuple_method { ($name: ident: ($($T:ident),+)) => { pub fn $name(&mut self) -> ($($T),+) { ($(replace_expr!($T self.n())),+) } } } macro_rules! tuple_methods { ($name:ident: ($($T:ident),+); $($rest:tt)*) => { tuple_method!($name:($($T),+)); tuple_methods!($($rest)*); }; () => () } macro_rules! vec_method { ($name: ident: ($($T:ty),+)) => { pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> { (0..n).map(|_|($(replace_expr!($T self.n())),+)).collect_vec() } }; ($name: ident: $T:ty) => { pub fn $name(&mut self, n: usize) -> Vec<$T> { (0..n).map(|_|self.n()).collect_vec() } }; } macro_rules! vec_methods { ($name:ident: ($($T:ty),+); $($rest:tt)*) => { vec_method!($name:($($T),+)); vec_methods!($($rest)*); }; ($name:ident: $T:ty; $($rest:tt)*) => { vec_method!($name:$T); vec_methods!($($rest)*); }; () => () } impl<R: BufRead> Reader<R> { pub fn new(reader: R) -> Reader<R> { let (buf, pos) = (Vec::new(), 0); Reader { reader, buf, pos } } prim_methods! { u: usize; i: i64; f: f64; str: String; c: char; string: String; u8; u16; u32; u64; u128; usize; i8; i16; i32; i64; i128; isize; f32; f64; char; } tuple_methods! { u2: (usize, usize); u3: (usize, usize, usize); u4: (usize, usize, usize, usize); i2: (i64, i64); i3: (i64, i64, i64); i4: (i64, i64, i64, i64); cuu: (char, usize, usize); } vec_methods! { uv: usize; uv2: (usize, usize); uv3: (usize, usize, usize); iv: i64; iv2: (i64, i64); iv3: (i64, i64, i64); vq: (char, usize, usize); }  pub fn n<T: FromStr>(&mut self) -> T where T::Err: Debug, { self.n_op().unwrap() }  pub fn n_op<T: FromStr>(&mut self) -> Option<T> where T::Err: Debug, { if self.buf.is_empty() { self._read_next_line(); } let mut start = None; while self.pos != self.buf.len() { match (self.buf[self.pos], start.is_some()) { (b' ', true) | (b'\n', true) => break, (_, true) | (b' ', false) => self.pos += 1, (b'\n', false) => self._read_next_line(), (_, false) => start = Some(self.pos), } } start.map(|s| from_utf8(&self.buf[s..self.pos]).unwrap().parse().unwrap()) }  fn _read_next_line(&mut self) { self.pos = 0; self.buf.clear(); self.reader.read_until(b'\n', &mut self.buf).unwrap(); } pub fn s(&mut self) -> Vec<char> { self.n::<String>().chars().collect() } pub fn digits(&mut self) -> Vec<i64> { self.n::<String>() .chars() .map(|c| (c as u8 - b'0') as i64) .collect() } pub fn char_map(&mut self, h: usize) -> Vec<Vec<char>> { (0..h).map(|_| self.s()).collect() } pub fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> { self.char_map(h) .iter() .map(|v| v.iter().map(|&c| c != ng).collect()) .collect() } pub fn matrix(&mut self, h: usize, w: usize) -> Vec<Vec<i64>> { (0..h).map(|_| self.iv(w)).collect() } } }

#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (h, w) = reader.u2();
    let s = reader.bool_map(h, '#');

    // let (h, w) = (500, 500);
    // let mut s = Vec::new();
    // for _ in 0..h {
    //     s.push(vec![false; w]);
    // }
    // s[0][0] = true;
    // s[h - 1][w - 1] = true;

    let grid = Grid::new(h, w, s);
    const INF: i64 = 1 << 62;
    let mut dist = vec![INF; h * w];
    dist[0] = 0;

    let mut q = VecDeque::new();
    let mut dq = VecDeque::new();
    q.push_front(0);
    for _ in 0.. {
        // walk
        if q.is_empty() {
            break;
        }
        while let Some(from) = q.pop_front() {
            let (x, y) = grid.xy(from);
            for dx in 0..3 {
                for dy in 0..3 {
                    if (dx as i64 - 1).abs() + (dy as i64 - 1).abs() < 2
                        && x + dx >= 1
                        && x + dx < grid.w + 1
                        && y + dy >= 1
                        && y + dy < grid.h + 1
                    {
                        let to = grid.key(x + dx - 1, y + dy - 1);
                        if dist[to] <= dist[from] {
                            continue;
                        }
                        if !*grid.get(to) {
                            dq.push_back(from);
                        } else {
                            dist[to] = dist[from];
                            q.push_back(to);
                        }
                    }
                }
            }
        }

        // destroy
        while let Some(from) = dq.pop_front() {
            let (x, y) = grid.xy(from);
            for dx in 0..5 {
                for dy in 0..5 {
                    if (dx as i64 - 2).abs() + (dy as i64 - 2).abs() < 4
                        && x + dx >= 2
                        && x + dx < grid.w + 2
                        && y + dy >= 2
                        && y + dy < grid.h + 2
                    {
                        let to = grid.key(x + dx - 2, y + dy - 2);
                        if dist[to] <= dist[from] + 1 {
                            continue;
                        }
                        dist[to] = dist[from] + 1;

                        q.push_back(to);
                    }
                }
            }
        }
    }

    println!("{}", dist[h * w - 1]);
    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{}", dist[grid.key(x, y)]);
    //     }
    //     println!();
    // }
    // dbg!(dist, q, dq);
}

#[allow(unused_imports)]
use grid::*;

#[allow(dead_code)]
mod grid {
    #[derive(Debug)]
    pub struct Grid<T> {
        pub h: usize,
        pub w: usize,
        pub max: usize,
        pub map: Vec<T>,
    }

    impl<T: Clone> Grid<T> {
        pub fn new(h: usize, w: usize, input: Vec<Vec<T>>) -> Grid<T> {
            let mut map = Vec::new();
            for r in input {
                for c in r {
                    map.push(c);
                }
            }
            let max = h * w;
            Grid { h, w, max, map }
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
        pub fn neighbor(&self, key: usize) -> Vec<usize> {
            let mut ret = self.one_way(key);
            if self.x(key) > 0 {
                ret.push(key - 1);
            }
            if self.y(key) > 0 {
                ret.push(key - self.w);
            }
            ret
        }

        pub fn one_way(&self, key: usize) -> Vec<usize> {
            let mut ret = Vec::new();
            if self.x(key) + 1 < self.w {
                ret.push(key + 1);
            }
            if self.y(key) + 1 < self.h {
                ret.push(key + self.w);
            }
            ret
        }
    }
}
