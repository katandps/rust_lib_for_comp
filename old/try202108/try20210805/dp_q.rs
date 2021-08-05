pub use reader::*;
#[allow(unused_imports)]
use {
    itertools::Itertools,
    num::Integer,
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
pub mod reader { #[allow(unused_imports)]use itertools::Itertools;use std::{fmt::Debug, io::*, str::*};pub struct Reader<R: BufRead> {reader: R,buf: Vec<u8>,pos: usize,}  macro_rules! prim_method { ($name:ident: $T: ty) => { pub fn $name(&mut self) -> $T { self.n::<$T>() } }; ($name:ident) => { prim_method!($name: $name); }; } macro_rules! prim_methods { ($name:ident: $T:ty; $($rest:tt)*) => { prim_method!($name:$T); prim_methods!($($rest)*); }; ($name:ident; $($rest:tt)*) => { prim_method!($name); prim_methods!($($rest)*); }; () => () }  macro_rules! replace_expr { ($_t:tt $sub:expr) => { $sub }; } macro_rules! tuple_method { ($name: ident: ($($T:ident),+)) => { pub fn $name(&mut self) -> ($($T),+) { ($(replace_expr!($T self.n())),+) } } } macro_rules! tuple_methods { ($name:ident: ($($T:ident),+); $($rest:tt)*) => { tuple_method!($name:($($T),+)); tuple_methods!($($rest)*); }; () => () } macro_rules! vec_method { ($name: ident: ($($T:ty),+)) => { pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> { (0..n).map(|_|($(replace_expr!($T self.n())),+)).collect_vec() } }; ($name: ident: $T:ty) => { pub fn $name(&mut self, n: usize) -> Vec<$T> { (0..n).map(|_|self.n()).collect_vec() } }; } macro_rules! vec_methods { ($name:ident: ($($T:ty),+); $($rest:tt)*) => { vec_method!($name:($($T),+)); vec_methods!($($rest)*); }; ($name:ident: $T:ty; $($rest:tt)*) => { vec_method!($name:$T); vec_methods!($($rest)*); }; () => () } impl<R: BufRead> Reader<R> {pub fn new(reader: R) -> Reader<R> {let (buf, pos) = (Vec::new(), 0);Reader { reader, buf, pos }} prim_methods! { u: usize; i: i64; f: f64; str: String; c: char; string: String; u8; u16; u32; u64; u128; usize; i8; i16; i32; i64; i128; isize; f32; f64; char; } tuple_methods! { u2: (usize, usize); u3: (usize, usize, usize); u4: (usize, usize, usize, usize); i2: (i64, i64); i3: (i64, i64, i64); i4: (i64, i64, i64, i64); cuu: (char, usize, usize); } vec_methods! { uv: usize; uv2: (usize, usize); uv3: (usize, usize, usize); iv: i64; iv2: (i64, i64); iv3: (i64, i64, i64); vq: (char, usize, usize); }  pub fn n<T: FromStr>(&mut self) -> T where T::Err: Debug, { self.n_op().unwrap() }pub fn n_op<T: FromStr>(&mut self) -> Option<T> where T::Err: Debug, {if self.buf.is_empty() { self._read_next_line(); }let mut start = None;while self.pos != self.buf.len() {match (self.buf[self.pos], start.is_some()) {(b' ', true) | (b'\n', true) => break,(_, true) | (b' ', false) => self.pos += 1,(b'\n', false) => self._read_next_line(),(_, false) => start = Some(self.pos),}}start.map(|s| from_utf8(&self.buf[s..self.pos]).unwrap().parse().unwrap())}fn _read_next_line(&mut self) {self.pos = 0;self.buf.clear();self.reader.read_until(b'\n', &mut self.buf).unwrap();}pub fn s(&mut self) -> Vec<char> { self.n::<String>().chars().collect() }pub fn char_map(&mut self, h: usize) -> Vec<Vec<char>> { (0..h).map(|_| self.s()).collect() }pub fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> { self.char_map(h).iter().map(|v| v.iter().map(|&c| c != ng).collect()).collect() }pub fn matrix(&mut self, h: usize, w: usize) -> Vec<Vec<i64>> { (0..h).map(|_| self.iv(w)).collect() }}}

#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let n = reader.u();
    let h = reader.uv(n);
    let a = reader.iv(n);

    let mut dp = SegmentTree::new(&vec![0; n + 1]);
    for i in 0..n {
        let (ai, hi) = (a[i], h[i]);
        let t = dp.get(0, hi + 1);
        dp.set(hi, t + ai);
    }
    println!("{}", dp.get(0, n + 1));
}

impl Monoid for i64 {
    fn ident() -> Self {
        0
    }

    fn initial() -> Self {
        0
    }

    fn op(&self, rhs: &Self) -> Self {
        max(*self, *rhs)
    }
}

#[allow(unused_imports)]
use segment_tree::*;

#[allow(dead_code)]
pub mod segment_tree {
    use std::fmt::Debug;

    /// 最小値を求めるセグメント木
    #[derive(Clone, Debug)]
    pub struct SegmentTree<M> {
        n: usize,
        node: Vec<M>,
    }

    pub trait Monoid: Debug + Clone + Copy {
        fn ident() -> Self;
        fn initial() -> Self;
        fn op(&self, rhs: &Self) -> Self;
    }

    impl<M: Monoid> SegmentTree<M> {
        pub fn new(v: &Vec<M>) -> Self {
            let size = v.len();
            let mut n = 1;
            while n < size {
                n *= 2
            }
            let mut node = vec![M::initial(); 2 * n - 1];
            for i in 0..size {
                node[i + n - 1] = v[i]
            }
            for i in (0..n - 1).rev() {
                node[i] = node[2 * i + 1].op(&node[2 * i + 2]);
            }
            Self { n, node }
        }

        /// index の値をvalに更新する
        pub fn set(&mut self, mut index: usize, val: M) {
            index += self.n - 1;
            self.node[index] = val;

            while index > 0 {
                index = (index - 1) / 2;
                self.node[index] = self.node[2 * index + 1].op(&self.node[2 * index + 2]);
            }
        }

        /// get for [a, b)
        pub fn get(&self, a: usize, b: usize) -> M {
            self.g(a, b, None, None, None)
        }

        fn g(&self, a: usize, b: usize, k: Option<usize>, l: Option<usize>, r: Option<usize>) -> M {
            let (k, l, r) = (k.unwrap_or(0), l.unwrap_or(0), r.unwrap_or(self.n));
            if r <= a || b <= l {
                M::initial()
            } else if a <= l && r <= b {
                self.node[k]
            } else {
                let vl = self.g(a, b, Some(2 * k + 1), Some(l), Some((l + r) / 2));
                let vr = self.g(a, b, Some(2 * k + 2), Some((l + r) / 2), Some(r));
                vl.op(&vr)
            }
        }
    }
}
