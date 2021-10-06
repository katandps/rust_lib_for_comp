//! 最小全域木
use super::Graph;
use crate::graph::{Edge, Weight};
use crate::*;

pub struct Prim(pub Weight);
impl Prim {
    ///
    /// Prim法でMinimumSpanningTree(最小全域木)を求める
    /// startと連結でない点は無視する
    /// ## 計算量
    /// 頂点数をV、辺数をEとすると
    /// 二分ヒープによる実装なのでO(ElogV)
    /// ```
    /// use library::graph::Graph;
    /// use library::graph::minimum_spanning_tree::Prim;
    /// let data = vec![
    ///     vec![-1, 2, 3, 1, -1],
    ///     vec![2, -1, -1, 4, -1],
    ///     vec![3, -1, -1, 1, 1],
    ///     vec![1, 4, 1, -1, 3],
    ///     vec![-1, -1, 1, 3, -1],
    /// ];
    ///
    /// let graph = Graph::from(&data);
    /// assert_eq!(5, Prim::build(&graph, 0).0);
    /// ```
    ///
    pub fn build(graph: &Graph, start: usize) -> Self {
        let mut t = Vec::new();
        let mut total: Weight = 0;
        let mut visits = vec![false; graph.n];
        let mut q = BinaryHeap::new();
        q.push(Reverse(Edge::new(graph.n, start, 0)));
        while let Some(Reverse(edge)) = q.pop() {
            if visits[edge.dst as usize] {
                continue;
            }
            visits[edge.dst as usize] = true;
            total += edge.weight;
            if edge.src != graph.n {
                t.push(edge)
            }
            graph.edges[edge.dst].iter().for_each(|f| {
                if !visits[f.dst as usize] {
                    q.push(Reverse(*f));
                }
            });
        }
        Prim(total)
    }
}
