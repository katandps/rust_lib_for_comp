//! # 最小全域木(クラスカル法)
//! 最小全域木(最小全域森)
//!
//! Kruskal法でMinimumSpanningTree(最小全域木)を求める
//! ## 計算量
//! 頂点数をV、辺数をEとすると $E\log E$
//! ```
//! use adjacency_matrix::GraphMatrix;
//! use kruskal::Kruskal;
//!
//! let graph = GraphMatrix::new(vec![
//!     vec![-1, 2, 3, 1, -1],
//!     vec![2, -1, -1, 4, -1],
//!     vec![3, -1, -1, 1, 1],
//!     vec![1, 4, 1, -1, 3],
//!     vec![-1, -1, 1, 3, -1],
//! ], Some(-1));
//! assert_eq!(5, Kruskal::from(&graph).sum());
//! ```
//!
use algebra::Zero;
use graph::GraphTrait;
use prelude::*;
use union_find::UnionFind;

#[codesnip::entry("kruskal", doc_hidden)]
pub struct Kruskal<W> {
    tree: Vec<(usize, usize, W)>,
    sum: W,
}

#[codesnip::entry("kruskal", doc_hidden)]
impl<W, G> From<&G> for Kruskal<W>
where
    W: Zero + PartialOrd + Copy + AddAssign,
    G: GraphTrait<Weight = W>,
{
    fn from(graph: &G) -> Self {
        let mut edges = (0..graph.size())
            .flat_map(|src| {
                graph
                    .edges(src)
                    .into_iter()
                    .map(|(dst, weight)| (src, dst, weight))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        edges.sort_by(|a, b| {
            (a.2)
                .partial_cmp(&b.2)
                .expect("辺のweightがソートできません")
        });
        let mut tree = Vec::new();
        let mut sum = W::zero();
        let mut uf = UnionFind::new(graph.size());
        for (src, dst, weight) in edges {
            if uf.unite(src, dst) {
                sum += weight;
                tree.push((src, dst, weight));
            }
        }
        Self { tree, sum }
    }
}

#[codesnip::entry("kruskal", doc_hidden)]
/// # 最小全域木を返す
/// Vec<(Src, Dst, Weight)> を返す
impl<W> Kruskal<W> {
    pub fn tree(&self) -> &Vec<(usize, usize, W)> {
        &self.tree
    }
}

#[codesnip::entry("kruskal", doc_hidden)]
impl<W: Copy> Kruskal<W> {
    pub fn sum(&self) -> W {
        self.sum
    }
}
