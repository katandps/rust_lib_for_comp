pub use reader::*;
#[allow(unused_imports)]
use {
    itertools::Itertools,
    num::Integer,
    proconio::fastout,
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

#[fastout]
pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let n = reader.u();
    let ab = reader.uv2(n - 1);
    let q = reader.u();
    let tex = reader.uv3(q);

    let mut g = vec![Vec::new(); n + 1];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }

    let mut lca = LowestCommonAncestor::new(&g, 1);

    let mut add = HashMap::new();
    let mut dp = vec![0; n + 1];

    for (t, e, x) in tex {
        let (a, b) = if t == 1 {
            ab[e - 1]
        } else {
            (ab[e - 1].1, ab[e - 1].0)
        };
        if lca.query(a, b) == b {
            *add.entry(a).or_insert(0) += x as i64;
        } else {
            dp[1] += x as i64;
            *add.entry(b).or_insert(0) -= x as i64;
        }
    }
    dp[1] += add.get(&1).unwrap_or(&0);
    f(1, 0, &g, &mut dp, &add);
    for i in 1..=n {
        println!("{}", dp[i]);
    }
}

fn f(cur: usize, parent: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<i64>, add: &HashMap<usize, i64>) {
    for &to in &g[cur] {
        if to == parent {
            continue;
        }
        dp[to] = dp[cur] + add.get(&to).unwrap_or(&0);
        f(to, cur, g, dp, add);
    }
}
#[allow(unused_imports)]
use lowest_common_ancestor::LowestCommonAncestor;

#[allow(dead_code)]
mod lowest_common_ancestor {
    use std::mem::swap;

    /// LowestCommonAncestor(LCA)を求めるライブラリ
    /// 事前処理NlogN、クエリlogN
    pub struct LowestCommonAncestor {
        parent: Vec<Vec<usize>>,
        dist: Vec<usize>,
    }

    impl LowestCommonAncestor {
        pub fn new(graph: &Vec<Vec<usize>>, root: usize) -> LowestCommonAncestor {
            let v = graph.len();
            let mut k = 1;
            while (1 << k) < v {
                k += 1;
            }
            let mut lca = LowestCommonAncestor {
                parent: vec![vec![std::usize::MAX; v]; k],
                dist: vec![std::usize::MAX; v],
            };
            lca.dfs(graph, root, std::usize::MAX, 0);
            for k in 0..k - 1 {
                for v in 0..v {
                    if lca.parent[k][v] == std::usize::MAX {
                        lca.parent[k + 1][v] = 1;
                    } else {
                        lca.parent[k + 1][v] = lca.parent[k][lca.parent[k][v]];
                    }
                }
            }
            lca
        }

        /// graph: グラフ
        /// v: 今見ている頂点
        /// p: parent
        /// d: 根からの距離
        fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize, d: usize) {
            self.parent[0][v] = p;
            self.dist[v] = d;
            for &to in &graph[v] {
                if to != p {
                    self.dfs(graph, to, v, d + 1);
                }
            }
        }

        /// u,vの最近共通祖先(LCA)を求める(O(logN))
        pub fn query(&mut self, mut u: usize, mut v: usize) -> usize {
            // uの深さはvの深さ以上
            if self.dist[u] < self.dist[v] {
                swap(&mut u, &mut v);
            }
            let k = self.parent.len();
            // LCAまでの距離を揃える
            for k in 0..k {
                if (self.dist[u] - self.dist[v]) >> k & 1 == 1 {
                    u = self.parent[k][u];
                }
            }
            if u == v {
                u
            } else {
                for i in 0..k {
                    let k = k - i - 1;
                    if self.parent[k][u] != self.parent[k][v] {
                        u = self.parent[k][u];
                        v = self.parent[k][v];
                    }
                }
                self.parent[0][u]
            }
        }

        /// 2頂点u,v間の距離を求める
        pub fn get_dist(&mut self, u: usize, v: usize) -> usize {
            let lca = self.query(u, v);
            self.dist[u] + self.dist[v] - 2 * self.dist[lca]
        }

        /// u,vを結ぶパス上に頂点aが存在するかどうか
        pub fn on_path(&mut self, u: usize, v: usize, a: usize) -> bool {
            self.get_dist(u, a) + self.get_dist(a, v) == self.get_dist(u, v)
        }
    }
}
