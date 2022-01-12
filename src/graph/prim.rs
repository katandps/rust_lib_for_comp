//! # 最小全域木(プリム法)
//! Prim法でMinimumSpanningTree(最小全域木)を求める
//! startと連結でない点は無視する
//! ## 計算量
//!
//! 二分ヒープによる実装なので頂点数をV、辺数をEとして$` O(E \log V)`$
//! ```
//! use rust_lib_for_comp::graph::adjacency_matrix::GraphMatrix;
//! use rust_lib_for_comp::graph::prim::Prim;
//!
//! let graph = GraphMatrix::from(vec![
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
use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "prim", doc_hidden)]
pub struct Prim<W> {
    tree: Vec<(usize, usize, W)>,
    sum: W,
}

#[snippet(name = "prim", doc_hidden)]
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
        q.push((Reverse(W::zero()), graph.size(), start));
        while let Some((Reverse(weight), src, dst)) = q.pop() {
            if visits[dst] {
                continue;
            }
            visits[dst] = true;
            sum += weight;
            if src != graph.size() {
                tree.push((src, dst, weight))
            }
            for (dst2, weight2) in graph.edges(dst) {
                if !visits[dst2] {
                    q.push((Reverse(weight2), dst, dst2));
                }
            }
        }
        Prim { tree, sum }
    }
}

#[snippet(name = "prim", doc_hidden)]
/// # 最小全域木を返す
/// Vec<(Src, Dst, Weight)> を返す
impl<W> Prim<W> {
    pub fn tree(&self) -> &Vec<(usize, usize, W)> {
        &self.tree
    }
}

#[snippet(name = "prim", doc_hidden)]
impl<W: Copy> Prim<W> {
    pub fn sum(&self) -> W {
        self.sum
    }
}
