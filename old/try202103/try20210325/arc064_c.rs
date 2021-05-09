#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (xs, ys, xt, yt) = reader.i4();
    let n = reader.u();
    let xyr = reader.iv3(n);

    let mut dists = vec![vec![0.0f64; n + 2]; n + 2];
    dists[0][n + 1] = (((xs - xt) * (xs - xt) + (ys - yt) * (ys - yt)) as f64).sqrt();
    dists[n + 1][0] = (((xs - xt) * (xs - xt) + (ys - yt) * (ys - yt)) as f64).sqrt();
    for i in 0..n {
        let (x, y, r) = xyr[i];
        let mut d = (((x - xs) * (x - xs) + (y - ys) * (y - ys)) as f64).sqrt() - r as f64;
        if d < 0.0 {
            d = 0.0
        }
        dists[0][i + 1] = d;
        dists[i + 1][0] = d;
    }
    for i in 0..n {
        let (x, y, r) = xyr[i];
        let mut d = (((x - xt) * (x - xt) + (y - yt) * (y - yt)) as f64).sqrt() - r as f64;
        if d < 0.0 {
            d = 0.0
        }
        dists[n + 1][i + 1] = d;
        dists[i + 1][n + 1] = d;
    }
    for i in 0..n {
        let (xi, yi, ri) = xyr[i];
        for j in 0..n {
            let (xj, yj, rj) = xyr[j];
            let mut d =
                (((xi - xj) * (xi - xj) + (yi - yj) * (yi - yj)) as f64).sqrt() - (ri + rj) as f64;
            if d < 0.0 {
                d = 0.0
            }
            dists[i + 1][j + 1] = d;
            dists[j + 1][i + 1] = d;
        }
    }

    let mut touch = vec![false; n + 2];
    touch[0] = true;

    let mut d = dists[0].clone();
    let mut cur;
    for _ in 0..n + 1 {
        let (&dist, to) = d
            .iter()
            .zip(0..n + 2)
            .filter(|(_d, i)| !touch[*i])
            .sorted_by(|(ad, _), (bd, _)| ad.partial_cmp(bd).unwrap())
            .next()
            .unwrap();

        // dbg!(&dist, &to);
        cur = dist;
        touch[to] = true;
        for i in 1..n + 2 {
            if d[i] > cur + dists[to][i] {
                d[i] = cur + dists[to][i];
            }
        }
    }
    println!("{}", d[n + 1]);
    // dbg!(&dists, &d);
}

pub use reader::*;
#[allow(unused_imports)]
use {
    itertools::Itertools,
    num::Integer,
    std::{cmp::*, collections::*, io::*, num::*, str::*},
};

#[allow(dead_code)]
#[rustfmt::skip]
pub mod reader {
    #[allow(unused_imports)]
    use itertools::Itertools;
    use std::{fmt::Debug, io::*, str::*};

    pub struct Reader<R: BufRead> { reader: R, buf: Vec<u8>, pos: usize }  macro_rules! prim_method { ($name:ident: $T: ty) => { pub fn $name(&mut self) -> $T { self.n::<$T>() } }; ($name:ident) => { prim_method!($name: $name); }; } macro_rules! prim_methods { ($name:ident: $T:ty; $($rest:tt)*) => { prim_method!($name:$T); prim_methods!($($rest)*); }; ($name:ident; $($rest:tt)*) => { prim_method!($name); prim_methods!($($rest)*); }; () => () }  macro_rules! replace_expr { ($_t:tt $sub:expr) => { $sub }; } macro_rules! tuple_method { ($name: ident: ($($T:ident),+)) => { pub fn $name(&mut self) -> ($($T),+) { ($(replace_expr!($T self.n())),+) } } } macro_rules! tuple_methods { ($name:ident: ($($T:ident),+); $($rest:tt)*) => { tuple_method!($name:($($T),+)); tuple_methods!($($rest)*); }; () => () } macro_rules! vec_method { ($name: ident: ($($T:ty),+)) => { pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> { (0..n).map(|_|($(replace_expr!($T self.n())),+)).collect_vec() } }; ($name: ident: $T:ty) => { pub fn $name(&mut self, n: usize) -> Vec<$T> { (0..n).map(|_|self.n()).collect_vec() } }; } macro_rules! vec_methods { ($name:ident: ($($T:ty),+); $($rest:tt)*) => { vec_method!($name:($($T),+)); vec_methods!($($rest)*); }; ($name:ident: $T:ty; $($rest:tt)*) => { vec_method!($name:$T); vec_methods!($($rest)*); }; () => () } impl<R: BufRead> Reader<R> {
    pub fn new(reader: R) -> Reader<R> {
        let (buf, pos) = (Vec::new(), 0);
        Reader { reader, buf, pos }
    } prim_methods! { u: usize; i: i64; f: f64; str: String; c: char; string: String; u8; u16; u32; u64; u128; usize; i8; i16; i32; i64; i128; isize; f32; f64; char; } tuple_methods! { u2: (usize, usize); u3: (usize, usize, usize); u4: (usize, usize, usize, usize); i2: (i64, i64); i3: (i64, i64, i64); i4: (i64, i64, i64, i64); cuu: (char, usize, usize); } vec_methods! { uv: usize; uv2: (usize, usize); uv3: (usize, usize, usize); iv: i64; iv2: (i64, i64); iv3: (i64, i64, i64); vq: (char, usize, usize); }  pub fn n<T: FromStr>(&mut self) -> T where T::Err: Debug, { self.n_op().unwrap() }
    pub fn n_op<T: FromStr>(&mut self) -> Option<T> where T::Err: Debug, {
        if self.buf.is_empty() { self._read_next_line(); }
        let mut start = None;
        while self.pos != self.buf.len() {
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) => self.pos += 1,
                (b'\n', false) => self._read_next_line(),
                (_, false) => start = Some(self.pos),
            }
        }
        start.map(|s| from_utf8(&self.buf[s..self.pos]).unwrap().parse().unwrap())
    }
    fn _read_next_line(&mut self) {
        self.pos = 0;
        self.buf.clear();
        self.reader.read_until(b'\n', &mut self.buf).unwrap();
    }
    pub fn s(&mut self) -> Vec<char> { self.n::<String>().chars().collect() }
    pub fn char_map(&mut self, h: usize) -> Vec<Vec<char>> { (0..h).map(|_| self.s()).collect() }
    pub fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> { self.char_map(h).iter().map(|v| v.iter().map(|&c| c != ng).collect()).collect() }
    pub fn matrix(&mut self, h: usize, w: usize) -> Vec<Vec<i64>> { (0..h).map(|_| self.iv(w)).collect() }
}
}
