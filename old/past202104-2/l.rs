#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let (n, m) = reader.u2();
    let pxpy = reader.iv2(n);
    let cxcyr = reader.iv3(m);

    const INF: f64 = std::usize::MAX as f64;
    let mut ans = INF;

    for p in 0..(1 << m) {
        let mut u = vec![false; m];
        for i in 0..m {
            if (p >> i) % 2 == 1 {
                u[i] = true;
            }
        }
        let mut weights = vec![vec![INF; n + m]; n + m];

        for i in 0..n {
            for j in i..n {
                if i == j {
                    weights[i][j] = 0.0;
                    weights[j][i] = 0.0;
                    continue;
                }
                let (ix, iy) = pxpy[i];
                let (jx, jy) = pxpy[j];
                let w = (((ix - jx) * (ix - jx) + (iy - jy) * (iy - jy)) as f64).sqrt();
                weights[i][j] = w;
                weights[j][i] = w;
            }
        }

        for i in 0..n {
            for j in 0..m {
                if !u[j] {
                    continue;
                }
                let (ix, iy) = pxpy[i];
                let (jx, jy, jr) = cxcyr[j];
                let w = ((((ix - jx) * (ix - jx) + (iy - jy) * (iy - jy)) as f64).sqrt()
                    - jr as f64)
                    .abs();
                weights[i][j + n] = w;
                weights[j + n][i] = w;
            }
        }
        for i in 0..m {
            for j in i..m {
                if !u[i] || !u[j] {
                    continue;
                }
                if i == j {
                    weights[i + n][j + n] = 0.0;
                    weights[j + n][i + n] = 0.0;
                    continue;
                }
                let (ix, iy, ir) = cxcyr[i];
                let (jx, jy, jr) = cxcyr[j];

                let d = (((ix - jx) * (ix - jx) + (iy - jy) * (iy - jy)) as f64).sqrt();
                let w = if ((ir + jr) as f64) < d {
                    d - (ir + jr) as f64
                } else if (ir - jr).abs() as f64 > d {
                    (ir - jr).abs() as f64 - d
                } else {
                    0.0
                };
                weights[i + n][j + n] = w;
                weights[j + n][i + n] = w;
            }
        }
        for i in 0..m {
            if !u[i] {
                weights[0][n + i] = 0.0;
                weights[n + i][0] = 0.0;
            }
        }

        let graph = Graph::from_matrix(&weights, n + m);
        let k = graph.prim(0);
        if ans.partial_cmp(&k).unwrap() == Ordering::Greater {
            ans = k;
        }
    }
    println!("{}", ans);
}

#[allow(unused_imports)]
use prim::*;

#[allow(dead_code)]
mod prim {
    use super::*;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    const INF: Weight = 1_000_000_000.0;

    ///
    /// Prim法でMinimumSpanningTree(最小全域木)を求める
    /// rから開始する (= rと連結でない点は無視する)
    ///
    impl Graph {
        pub fn prim(&self, r: i64) -> Weight {
            let mut t = Edges(Vec::new());
            let mut total: Weight = 0.0;
            let mut vis = vec![false; self.len()];
            let mut q = BinaryHeap::new();
            q.push(Reverse(Edge::new(-1, r, 0.0)));
            while !q.is_empty() {
                let Reverse(e) = q.pop().unwrap();
                if vis[e.dst as usize] {
                    continue;
                }
                vis[e.dst as usize] = true;
                total += e.weight;
                if e.src != -1 {
                    t.0.push(e)
                }
                for f in &self.0[e.dst as usize].0 {
                    if !vis[f.dst as usize] {
                        q.push(Reverse(*f));
                    }
                }
            }
            total
        }
    }
}

#[allow(unused_imports)]
use graph::*;

#[allow(dead_code)]
pub mod graph {
    use std::cmp::Ordering;

    pub type Weight = f64;

    #[derive(Copy, Clone)]
    pub struct Edge {
        pub src: i64,
        pub dst: i64,
        pub weight: Weight,
    }

    impl Edge {
        pub fn default() -> Edge {
            let (src, dst, weight) = (0, 0, 0.0);
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
            self.weight.partial_cmp(&other.weight).unwrap()
        }
    }

    #[derive(Clone)]
    pub struct Edges(pub(crate) Vec<Edge>);

    pub struct Graph(pub(crate) Vec<Edges>);

    pub struct Array(Vec<Weight>);

    pub struct Matrix(Vec<Array>);

    impl Graph {
        pub fn from_matrix(weights: &Vec<Vec<Weight>>, n: usize) -> Graph {
            let mut ret = Graph(vec![Edges(Vec::new()); n]);
            for i in 0..n as i64 {
                for j in i + 1..n as i64 {
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
            self.0[a as usize].0.push(Edge::new(a, b, w));
            self.0[b as usize].0.push(Edge::new(b, a, w));
        }

        pub fn add_arc(graph: &mut Graph, a: i64, b: i64, w: Weight) {
            graph.0[a as usize].0.push(Edge::new(a, b, w));
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
