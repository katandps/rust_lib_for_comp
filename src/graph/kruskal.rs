//! 最小全域木(クラスカル法)
//! 最小全域木(最小全域森)
//!
//! Kruskal法でMinimumSpanningTree(最小全域木)を求める
//! ## 計算量
//! 頂点数をV、辺数をEとすると $` E \log E`$
//! ```
//! use rust_lib_for_comp::graph::adjacency_list::Graph;
//! use rust_lib_for_comp::graph::kruskal::Kruskal;
//!
//! let graph = Graph::from(&vec![
//!     vec![-1, 2, 3, 1, -1],
//!     vec![2, -1, -1, 4, -1],
//!     vec![3, -1, -1, 1, 1],
//!     vec![1, 4, 1, -1, 3],
//!     vec![-1, -1, 1, 3, -1],
//! ]);
//! assert_eq!(5, Kruskal::from(&graph).sum());
//! ```
//!
use super::GraphTrait;
use crate::algebra::Zero;
use crate::data_structure::union_find::UnionFind;
use crate::graph::Edge;
use crate::prelude::*;

#[snippet(name = "kruskal", doc_hidden)]
pub struct Kruskal<W> {
    tree: Vec<Edge<W>>,
    sum: W,
}

#[snippet(name = "kruskal", doc_hidden)]
impl<W, G> From<&G> for Kruskal<W>
where
    W: Zero + PartialOrd + Copy + AddAssign,
    G: GraphTrait<Weight = W>,
{
    fn from(graph: &G) -> Self {
        let mut edges = Vec::new();
        for i in 0..graph.size() {
            for e in graph.edges(i) {
                edges.push(e);
            }
        }
        edges.sort_by(|a, b| a.partial_cmp(b).expect("辺のweightがソートできません"));
        let mut tree = Vec::new();
        let mut sum = W::zero();
        let mut uf = UnionFind::new(graph.size());
        for edge in edges {
            if uf.root(edge.src) != uf.root(edge.dst) {
                uf.unite(edge.src, edge.dst);
                sum += edge.weight;
                tree.push(edge);
            }
        }
        Self { tree, sum }
    }
}

#[snippet(name = "kruskal", doc_hidden)]
impl<W> Kruskal<W> {
    pub fn tree(&self) -> &Vec<Edge<W>> {
        &self.tree
    }
}

#[snippet(name = "kruskal", doc_hidden)]
impl<W: Copy> Kruskal<W> {
    pub fn sum(&self) -> W {
        self.sum
    }
}
