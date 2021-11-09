//! #最小全域木(プリム法)
//! Prim法でMinimumSpanningTree(最小全域木)を求める
//! startと連結でない点は無視する
//! ## 計算量
//!
//! 二分ヒープによる実装なので頂点数をV、辺数をEとして$` O(E \log V)`$
//! ```
//! use rust_competitive_programming::graph::Graph;
//! use rust_competitive_programming::graph::prim::Prim;
//!
//! let graph = Graph::from(&vec![
//!     vec![-1, 2, 3, 1, -1],
//!     vec![2, -1, -1, 4, -1],
//!     vec![3, -1, -1, 1, 1],
//!     vec![1, 4, 1, -1, 3],
//!     vec![-1, -1, 1, 3, -1],
//! ]);
//! assert_eq!(5, Prim::from(&graph).sum());
//! ```
//!
use crate::algebra::Zero;
use crate::graph::{Edge, GraphTrait};
use crate::prelude::*;

pub struct Prim<W> {
    tree: Vec<Edge<W>>,
    sum: W,
}

impl<W, G> From<&G> for Prim<W>
where
    W: Zero + Ord + Copy + AddAssign,
    G: GraphTrait<Weight = W>,
{
    fn from(graph: &G) -> Self {
        let start = 0;
        let mut tree = Vec::new();
        let mut sum = W::zero();
        let mut visits = vec![false; graph.size()];
        let mut q = BinaryHeap::new();
        q.push(Reverse(Edge::new(graph.size(), start, W::zero())));
        while let Some(Reverse(edge)) = q.pop() {
            if visits[edge.dst as usize] {
                continue;
            }
            visits[edge.dst as usize] = true;
            sum += edge.weight;
            if edge.src != graph.size() {
                tree.push(edge)
            }
            for edge in graph.edges(edge.dst) {
                if !visits[edge.dst as usize] {
                    q.push(Reverse(edge));
                }
            }
        }
        Prim { tree, sum }
    }
}

impl<W> Prim<W> {
    pub fn tree(&self) -> &Vec<Edge<W>> {
        &self.tree
    }
}

impl<W: Copy> Prim<W> {
    pub fn sum(&self) -> W {
        self.sum
    }
}
