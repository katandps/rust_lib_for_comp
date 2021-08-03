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

    // dp[number of 3][number of 2][number of 1] = p;
    let mut dp = vec![vec![vec![0.0; n + 1]; n + 1]; n + 1];

    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    for _ in 0..n {
        match reader.u() {
            1 => one += 1,
            2 => two += 1,
            3 => three += 1,
            _ => unreachable!(),
        }
    }

    println!("{}", s(one, two, three, &mut dp));
}

fn s(one: usize, two: usize, three: usize, dp: &mut Vec<Vec<Vec<f64>>>) -> f64 {
    if dp[three][two][one] != 0.0 {
        return dp[three][two][one];
    }
    if (one | two | three) == 0 {
        return 0.0;
    }
    let n = dp.len() as f64 - 1.0;
    let mut ret = 1.0;
    if one != 0 {
        ret += (one as f64 / n) * (s(one - 1, two, three, dp));
    }
    if two != 0 {
        ret += (two as f64 / n) * (s(one + 1, two - 1, three, dp));
    }
    if three != 0 {
        ret += (three as f64 / n) * (s(one, two + 1, three - 1, dp));
    }
    if (dp.len() - 1 - one - two - three) > 0 {
        ret *= n / (one + two + three) as f64;
    }
    dp[three][two][one] = ret;
    ret
}
