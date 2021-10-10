//! 最小全域木
use super::Graph;
use crate::algebra::Zero;
use crate::graph::Edge;
use crate::*;

/// Prim法で求めた最小全域木
pub struct Prim<W> {
    tree: Vec<Edge<W>>,
    sum: W,
}
impl<W: Zero + Ord + Copy + Add<Output = W> + AddAssign> Prim<W> {
    ///
    /// Prim法でMinimumSpanningTree(最小全域木)を求める
    /// startと連結でない点は無視する
    /// ## 計算量
    /// 頂点数をV、辺数をEとすると
    /// 二分ヒープによる実装なのでO(ElogV)
    /// ```
    /// use library::graph::Graph;
    /// use library::graph::minimum_spanning_tree::Prim;
    ///
    /// let graph = Graph::from(&vec![
    ///     vec![-1, 2, 3, 1, -1],
    ///     vec![2, -1, -1, 4, -1],
    ///     vec![3, -1, -1, 1, 1],
    ///     vec![1, 4, 1, -1, 3],
    ///     vec![-1, -1, 1, 3, -1],
    /// ]);
    /// assert_eq!(5, Prim::build(&graph, 0).sum());
    /// ```
    ///
    pub fn build(graph: &Graph<W>, start: usize) -> Self {
        let mut tree = Vec::new();
        let mut sum = W::zero();
        let mut visits = vec![false; graph.n];
        let mut q = BinaryHeap::new();
        q.push(Reverse(Edge::new(graph.n, start, W::zero())));
        while let Some(Reverse(edge)) = q.pop() {
            if visits[edge.dst as usize] {
                continue;
            }
            visits[edge.dst as usize] = true;
            sum += edge.weight;
            if edge.src != graph.n {
                tree.push(edge)
            }
            graph.edges[edge.dst].iter().for_each(|f| {
                if !visits[f.dst as usize] {
                    q.push(Reverse(f.clone()));
                }
            });
        }
        Prim { tree, sum }
    }

    pub fn tree(&self) -> &Vec<Edge<W>> {
        &self.tree
    }

    pub fn sum(&self) -> W {
        self.sum.clone()
    }
}
