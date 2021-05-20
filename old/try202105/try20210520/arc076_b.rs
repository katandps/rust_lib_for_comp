#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let n = reader.u();
    let xy = reader.iv2(n);

    let x = (0..)
        .zip(&xy)
        .map(|(i, (x, _y))| (x, i))
        .sorted()
        .collect_vec();
    let y = (0..)
        .zip(&xy)
        .map(|(i, (_x, y))| (y, i))
        .sorted()
        .collect_vec();

    let mut g = Graph::new(n);
    for i in 1..n {
        let (from_x, src) = x[i - 1];
        let (to_x, dst) = x[i];
        g.add_edge(src, dst, *to_x - *from_x);
        let (from_y, src) = y[i - 1];
        let (to_y, dst) = y[i];
        g.add_edge(src, dst, *to_y - *from_y);
    }
    println!("{}", g.prim(0));
}

#[allow(unused_imports)]
use graph::*;

#[allow(dead_code)]
pub mod graph {
    use std::cmp::Ordering;
    use std::fmt::{Debug, Formatter};

    pub type Weight = i64;

    #[derive(Copy, Clone)]
    pub struct Edge {
        pub src: i64,
        pub dst: i64,
        pub weight: Weight,
    }

    impl Debug for Edge {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} -> {} : {}", self.src, self.dst, self.weight)
        }
    }

    impl Edge {
        pub fn default() -> Edge {
            let (src, dst, weight) = (0, 0, 0);
            Edge { src, dst, weight }
        }

        pub fn new(src: i64, dst: i64, weight: Weight) -> Edge {
            Edge { src, dst, weight }
        }
    }

    impl std::cmp::PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            self.weight.eq(&other.weight)
        }
    }

    impl std::cmp::Eq for Edge {}

    impl std::cmp::PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.weight.partial_cmp(&other.weight)
        }
    }

    impl std::cmp::Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            self.weight.cmp(&other.weight)
        }
    }

    /// 辺の情報を使用してグラフの問題を解くためのライブラリ
    #[derive(Clone, Debug)]
    pub struct Graph(Vec<Vec<Edge>>);

    impl Graph {
        /// n: 頂点数
        pub fn new(n: usize) -> Self {
            Self(vec![Vec::new(); n])
        }

        /// 辺行列からグラフを生成する O(N^2)
        pub fn from_matrix(weights: &Vec<Vec<Weight>>, n: usize) -> Graph {
            let mut ret = Self::new(n);
            for i in 0..n as i64 {
                for j in i + 1..n as i64 {
                    if weights[i as usize][j as usize] == -1 {
                        continue;
                    }
                    ret.add_edge(i, j, weights[i as usize][j as usize]);
                    ret.add_edge(j, i, weights[j as usize][i as usize]);
                }
            }
            ret
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn add_edge(&mut self, a: i64, b: i64, w: Weight) {
            self.0[a as usize].push(Edge::new(a, b, w));
            self.0[b as usize].push(Edge::new(b, a, w));
        }

        pub fn add_arc(graph: &mut Graph, a: i64, b: i64, w: Weight) {
            graph.0[a as usize].push(Edge::new(a, b, w));
        }

        pub fn edges_from(&self, from: usize) -> &Vec<Edge> {
            &self.0[from]
        }
    }
}

#[allow(unused_imports)]
use prim::*;

#[allow(dead_code)]
mod prim {
    use super::*;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    const INF: Weight = 1_000_000_000;

    ///
    /// Prim法でMinimumSpanningTree(最小全域木)を求める
    /// rから開始する (= rと連結でない点は無視する)
    ///
    impl Graph {
        pub fn prim(&self, r: i64) -> Weight {
            let mut t = Vec::new();
            let mut total: Weight = 0;
            let mut visits = vec![false; self.len()];
            let mut q = BinaryHeap::new();
            q.push(Reverse(Edge::new(-1, r, 0)));
            while !q.is_empty() {
                let Reverse(e) = q.pop().unwrap();
                if visits[e.dst as usize] {
                    continue;
                }
                visits[e.dst as usize] = true;
                total += e.weight;
                if e.src != -1 {
                    t.push(e)
                }
                for f in self.edges_from(e.dst as usize).iter() {
                    if !visits[f.dst as usize] {
                        q.push(Reverse(*f));
                    }
                }
            }
            total
        }
    }
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
