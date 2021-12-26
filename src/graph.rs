//! # グラフ
//! 汎用的なグラフのTrait及び辺の構造体

use crate::prelude::*;

pub mod adjacency_list;
pub mod adjacency_matrix;
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
pub mod tree;
pub mod warshall_floyd;

/// Edge 辺
/// W はWeightで各処理に対応するTraitを実装する
#[snippet(name = "graph", doc_hidden)]
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
