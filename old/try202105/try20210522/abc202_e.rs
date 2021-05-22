#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let n = reader.u();
    let p = reader.uv(n - 1);
    let q = reader.u();
    let ud = reader.uv2(q);

    let mut g = vec![Vec::new(); n + 1];
    for i in 0..n - 1 {
        let from = i + 2;
        let to = p[i];
        g[min(from, to)].push(max(from, to));
    }

    let mut euler_tour = vec![0];

    let mut depth = vec![n + 1; n + 1];
    let mut depth_node = vec![BTreeSet::new(); n];
    depth[0] = 0;
    depth[1] = 0;
    let mut q = VecDeque::new();
    q.push_front(1);
    while !q.is_empty() {
        let from = q.pop_front().unwrap();
        for &to in &g[from] {
            if depth[to] > depth[from] {
                depth[to] = depth[from] + 1;
                q.push_front(to);
            }
        }
    }

    dfs(0, 1, &g, &mut euler_tour);
    euler_tour.push(0);

    let mut in_t = vec![euler_tour.len(); n + 1];
    let mut out_t = vec![euler_tour.len(); n + 1];
    for i in 0..euler_tour.len() {
        if in_t[euler_tour[i]] > i {
            in_t[euler_tour[i]] = i;
        } else if out_t[euler_tour[i]] > i {
            out_t[euler_tour[i]] = i;
        }
    }

    for i in 1..=n {
        depth_node[depth[i]].insert(in_t[i]);
    }

    let depth_node = depth_node
        .into_iter()
        .map(|btm| btm.into_iter().collect_vec())
        .collect_vec();

    for (u, d) in ud {
        if depth_node[d].len() == 0 {
            println!("0");
            continue;
        }

        let mut i_ok = -1;
        let mut i_ng = depth_node[d].len() as i32;
        while (i_ok - i_ng).abs() > 1 {
            let mid = (i_ok + i_ng) / 2;
            if depth_node[d][mid as usize] < in_t[u] {
                i_ok = mid;
            } else {
                i_ng = mid;
            }
        }
        let mut j_ok = -1;
        let mut j_ng = depth_node[d].len() as i32;
        while (j_ok - j_ng).abs() > 1 {
            let mid = (j_ok + j_ng) / 2;
            if depth_node[d][mid as usize] < out_t[u] {
                j_ok = mid;
            } else {
                j_ng = mid;
            }
        }

        println!("{}", j_ok - i_ok);
    }
}

fn dfs(parent: usize, current: usize, g: &Vec<Vec<usize>>, tour: &mut Vec<usize>) {
    tour.push(current);
    for &to in &g[current] {
        if to == parent {
            continue;
        }
        dfs(current, to, g, tour);
    }
    tour.push(current)
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

    pub struct Reader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        pos: usize,
    }  macro_rules! prim_method { ($name:ident: $T: ty) => { pub fn $name(&mut self) -> $T { self.n::<$T>() } }; ($name:ident) => { prim_method!($name: $name); }; } macro_rules! prim_methods { ($name:ident: $T:ty; $($rest:tt)*) => { prim_method!($name:$T); prim_methods!($($rest)*); }; ($name:ident; $($rest:tt)*) => { prim_method!($name); prim_methods!($($rest)*); }; () => () }  macro_rules! replace_expr { ($_t:tt $sub:expr) => { $sub }; } macro_rules! tuple_method { ($name: ident: ($($T:ident),+)) => { pub fn $name(&mut self) -> ($($T),+) { ($(replace_expr!($T self.n())),+) } } } macro_rules! tuple_methods { ($name:ident: ($($T:ident),+); $($rest:tt)*) => { tuple_method!($name:($($T),+)); tuple_methods!($($rest)*); }; () => () } macro_rules! vec_method { ($name: ident: ($($T:ty),+)) => { pub fn $name(&mut self, n: usize) -> Vec<($($T),+)> { (0..n).map(|_|($(replace_expr!($T self.n())),+)).collect_vec() } }; ($name: ident: $T:ty) => { pub fn $name(&mut self, n: usize) -> Vec<$T> { (0..n).map(|_|self.n()).collect_vec() } }; } macro_rules! vec_methods { ($name:ident: ($($T:ty),+); $($rest:tt)*) => { vec_method!($name:($($T),+)); vec_methods!($($rest)*); }; ($name:ident: $T:ty; $($rest:tt)*) => { vec_method!($name:$T); vec_methods!($($rest)*); }; () => () } impl<R: BufRead> Reader<R> {
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
