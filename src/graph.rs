//! # グラフ
//!

use crate::prelude::*;

pub mod bellman_ford;
pub mod bipartite_graph;
pub mod dijkstra;
pub mod directed_acyclic_graph;
pub mod grid;
pub mod kruskal;
pub mod lowest_common_ancestor;
pub mod prim;
pub mod retrograde_analysis;
pub mod strongly_connected_components;
pub mod warshall_floyd;

#[snippet(name = "graph", doc_hidden)]
/// Edge 辺
/// W はWeightで各処理に対応するTraitを実装する
#[derive(Copy, Clone, Eq, Default)]
pub struct Edge<W> {
    pub src: usize,
    pub dst: usize,
    pub weight: W,
}

#[snippet(name = "graph", doc_hidden)]
impl<W: Display> Debug for Edge<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} : {}", self.src, self.dst, self.weight)
    }
}

#[snippet(name = "graph", doc_hidden)]
impl<W> Edge<W> {
    pub fn new(src: usize, dst: usize, weight: W) -> Self {
        Edge { src, dst, weight }
    }
}

#[snippet(name = "graph", doc_hidden)]
impl<W: PartialEq> PartialEq for Edge<W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}

#[snippet(name = "graph", doc_hidden)]
impl<W: PartialOrd> PartialOrd for Edge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

#[snippet(name = "graph", doc_hidden)]
impl<W: PartialOrd + Eq> Ord for Edge<W> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(&other.weight).expect("Found NAN")
    }
}

#[snippet(name = "graph", doc_hidden)]
pub trait GraphTrait {
    type Weight;
    fn size(&self) -> usize;
    fn edges(&self, src: usize) -> Vec<Edge<Self::Weight>>;
    fn rev_edges(&self, dst: usize) -> Vec<Edge<Self::Weight>>;
    /// 各頂点の入次数を返す
    fn indegree(&self) -> Vec<i32> {
        (0..self.size())
            .map(|dst| self.rev_edges(dst).len() as i32)
            .collect()
    }

    /// 各頂点の出次数を返す
    fn outdegree(&self) -> Vec<i32> {
        (0..self.size())
            .map(|src| self.edges(src).len() as i32)
            .collect()
    }
}

#[snippet(name = "graph", doc_hidden)]
impl<W: Clone> GraphTrait for Graph<W> {
    type Weight = W;
    fn size(&self) -> usize {
        self.n
    }
    fn edges(&self, src: usize) -> Vec<Edge<W>> {
        self.edges[src].clone()
    }
    fn rev_edges(&self, src: usize) -> Vec<Edge<W>> {
        self.rev_edges[src].clone()
    }
}

#[snippet(name = "graph", doc_hidden)]
/// 辺の情報を使用してグラフの問題を解くためのライブラリ
pub struct Graph<W> {
    pub n: usize,
    pub edges: Vec<Vec<Edge<W>>>,
    pub rev_edges: Vec<Vec<Edge<W>>>,
}

#[snippet(name = "graph", doc_hidden)]
impl<W: Clone> Clone for Graph<W> {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            edges: self.edges.clone(),
            rev_edges: self.rev_edges.clone(),
        }
    }
}

#[snippet(name = "graph", doc_hidden)]
/// i64の隣接行列からグラフを生成する O(N^2)
impl From<&Vec<Vec<i64>>> for Graph<i64> {
    fn from(w: &Vec<Vec<i64>>) -> Self {
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

#[snippet(name = "graph", doc_hidden)]
impl<W: Clone> Graph<W> {
    /// n: 頂点数
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: vec![Vec::new(); n],
            rev_edges: vec![Vec::new(); n],
        }
    }
    /// 相互に行き来できる辺をつける
    pub fn add_edge(&mut self, a: usize, b: usize, w: W) {
        self.edges[a].push(Edge::new(a, b, w.clone()));
        self.edges[b].push(Edge::new(b, a, w.clone()));
        self.rev_edges[a].push(Edge::new(a, b, w.clone()));
        self.rev_edges[b].push(Edge::new(b, a, w));
    }

    /// 1方向にのみ移動できる辺をつける
    pub fn add_arc(&mut self, a: usize, b: usize, w: W) {
        self.edges[a].push(Edge::new(a, b, w.clone()));
        self.rev_edges[b].push(Edge::new(b, a, w));
    }
}
