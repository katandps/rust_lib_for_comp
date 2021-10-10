//! グラフ

use crate::*;

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

#[derive(Copy, Clone, Eq, Ord, Default)]
pub struct Edge<W> {
    pub src: usize,
    pub dst: usize,
    pub weight: W,
}

impl<W: Display> Debug for Edge<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} : {}", self.src, self.dst, self.weight)
    }
}

impl<W> Edge<W> {
    pub fn new(src: usize, dst: usize, weight: W) -> Self {
        Edge { src, dst, weight }
    }
}

impl<W: PartialEq> PartialEq for Edge<W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}

impl<W: PartialOrd> PartialOrd for Edge<W> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

pub trait GraphTrait {
    type Weight;
    fn size(&self) -> usize;
    fn edges(&self, src: usize) -> Vec<Edge<Self::Weight>>;
}

impl<W: Clone> GraphTrait for Graph<W> {
    type Weight = W;
    fn size(&self) -> usize {
        self.n
    }
    fn edges(&self, src: usize) -> Vec<Edge<W>> {
        self.edges[src].clone()
    }
}

/// 辺の情報を使用してグラフの問題を解くためのライブラリ
pub struct Graph<W> {
    pub n: usize,
    pub edges: Vec<Vec<Edge<W>>>,
    pub rev_edges: Vec<Vec<Edge<W>>>,
}

/// i64の辺行列からグラフを生成する O(N^2)
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
                ret.add_edge(i, j, w[i as usize][j as usize].clone());
                ret.add_edge(j, i, w[j as usize][i as usize].clone());
            }
        }
        ret
    }
}

impl<W> Graph<W> {
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
