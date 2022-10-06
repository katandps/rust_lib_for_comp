//! # グラフ
//! 一般のグラフのTrait

use crate::prelude::*;

pub mod adjacency_list;
pub mod adjacency_matrix;
pub mod bellman_ford;
pub mod bipartite_graph;
pub mod dijkstra;
pub mod directed_acyclic_graph;
pub mod find_cycle;
pub mod grid;
pub mod kruskal;
pub mod prim;
pub mod retrograde_analysis;
pub mod strongly_connected_components;
pub mod tree;
pub mod warshall_floyd;

#[snippet(name = "graph", doc_hidden)]
pub trait GraphTrait {
    type Weight;
    fn size(&self) -> usize;
    /// 引数で指定した頂点を始点とする辺の情報を返す
    fn edges(&self, src: usize) -> Vec<(usize, Self::Weight)>;
    /// 引数で指定した頂点を終点とする辺の情報を返す
    fn rev_edges(&self, dst: usize) -> Vec<(usize, Self::Weight)>;
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
