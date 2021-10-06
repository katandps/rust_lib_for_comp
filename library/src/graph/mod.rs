//! グラフライブラリ

use crate::*;

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

pub mod bellman_ford;
pub mod bipartite_graph;
pub mod dijkstra;
pub mod directed_acyclic_graph;
pub mod grid;
pub mod lowest_common_ancestor;
pub mod minimum_spanning_tree;
pub mod retrograde_analysis;
pub mod strongly_connected_components;
pub mod union_find;
pub mod warshall_floyd;

///////////////

pub type Weight = i64;

pub const INF: Weight = 1 << 60;

#[derive(Copy, Clone)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
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

    pub fn new(src: usize, dst: usize, weight: Weight) -> Edge {
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
pub struct Graph {
    pub n: usize,
    pub edges: Vec<Vec<Edge>>,
    pub rev_edges: Vec<Vec<Edge>>,
}

/// 辺行列からグラフを生成する O(N^2)
impl From<&Vec<Vec<Weight>>> for Graph {
    fn from(w: &Vec<Vec<Weight>>) -> Self {
        let n = w.len();
        let mut ret = Self::new(n);
        for i in 0..n {
            assert_eq!(n, w[i].len());
            for j in i + 1..n {
                if w[i][j] == -1 {
                    continue;
                }
                ret.add_edge(i, j, w[i as usize][j as usize]);
                ret.add_edge(j, i, w[j as usize][i as usize]);
            }
        }
        ret
    }
}

impl Graph {
    /// n: 頂点数
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: vec![Vec::new(); n],
            rev_edges: vec![Vec::new(); n],
        }
    }

    /// 相互に行き来できる辺をつける
    pub fn add_edge(&mut self, a: usize, b: usize, w: Weight) {
        self.edges[a].push(Edge::new(a, b, w));
        self.edges[b].push(Edge::new(b, a, w));
        self.rev_edges[a].push(Edge::new(a, b, w));
        self.rev_edges[b].push(Edge::new(b, a, w));
    }

    /// 1方向にのみ移動できる辺をつける
    pub fn add_arc(&mut self, a: usize, b: usize, w: Weight) {
        self.edges[a].push(Edge::new(a, b, w));
        self.rev_edges[b].push(Edge::new(b, a, w));
    }

    /// 各頂点の入次数を返す
    pub fn indegree(&self) -> Vec<i32> {
        (0..self.n)
            .map(|dst| self.rev_edges[dst].len() as i32)
            .collect()
    }

    /// 各頂点の出次数を返す
    pub fn outdegree(&self) -> Vec<i32> {
        (0..self.n)
            .map(|src| self.edges[src].len() as i32)
            .collect()
    }
}
