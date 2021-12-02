//! # 二部グラフ
//! グラフが二部グラフかどうか判定する
//! 二部グラフだったときはその分割方法を1つ返す
//! なお、グラフが連結でない場合は正しく判定できない
use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "bipartite-graph", doc_hidden)]
pub struct BipartiteGraph(Vec<bool>);

#[snippet(name = "bipartite-graph", doc_hidden)]
impl<G: GraphTrait> From<G> for BipartiteGraph {
    fn from(graph: G) -> Self {
        let mut dist = vec![None; graph.size()];
        dist[0] = Some(true);
        if Self::dfs(&graph, 0, &mut dist) {
            Self(dist.iter().map(|op| op.unwrap()).collect())
        } else {
            Self(Vec::new())
        }
    }
}

#[snippet(name = "bipartite-graph", doc_hidden)]
impl BipartiteGraph {
    pub fn is_bipartite(&self) -> bool {
        !self.0.is_empty()
    }

    fn dfs<G: GraphTrait>(graph: &G, src: usize, d: &mut Vec<Option<bool>>) -> bool {
        for edge in graph.edges(src) {
            if d[edge.dst] == d[src] {
                return false;
            }
            if d[edge.dst].is_none() {
                d[edge.dst] = d[src].map(|b| !b);
                if !Self::dfs(graph, edge.dst, d) {
                    return false;
                }
            }
        }
        true
    }
}
