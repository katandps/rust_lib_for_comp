//! # 二部グラフ
//! グラフが二部グラフかどうか判定する
//! 二部グラフだったときはその分割方法を1つ返す
use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "bipartite-graph", doc_hidden)]
pub trait BipartiteGraphTrait: GraphTrait {
    /// # 2部グラフとして分割する
    /// すべての頂点について、二部グラフとしてbooleanで彩色した結果を返す
    /// 塗り分けられなかった場合はNoneを返す
    /// 連結成分数でバランスしない
    fn bipartition(&self) -> Option<Vec<bool>> {
        let mut dist = vec![None; self.size()];
        for src in 0..self.size() {
            if dist[src].is_none() {
                dist[src] = Some(true);
                if !self.dfs(src, &mut dist) {
                    return None;
                }
            }
        }
        Some(dist.iter().map(|op| op.unwrap()).collect())
    }

    fn dfs(&self, src: usize, d: &mut Vec<Option<bool>>) -> bool {
        for (dst, _weight) in self.edges(src) {
            if d[dst] == d[src] {
                return false;
            }
            if d[dst].is_none() {
                d[dst] = d[src].map(|b| !b);
                if !self.dfs(dst, d) {
                    return false;
                }
            }
        }
        true
    }
}
#[snippet(name = "bipartite-graph", doc_hidden)]
impl<G: GraphTrait> BipartiteGraphTrait for G {}

#[cfg(test)]
mod test {
    use super::BipartiteGraphTrait;
    use crate::graph::adjacency_list::Graph;

    #[test]
    fn test_ok() {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (1, 4), (5, 6)];
        let mut graph = Graph::new(8);
        for &(a, b) in &edges {
            graph.add_edge(a, b, ());
        }

        let result = graph.bipartition();
        assert!(result.is_some());
        let result = result.unwrap();
        for (a, b) in edges {
            assert_ne!(result[a], result[b]);
        }
    }

    #[test]
    fn test_ng() {
        let edges = vec![(0, 1), (1, 2), (0, 2)];
        let mut graph = Graph::new(3);
        for &(a, b) in &edges {
            graph.add_edge(a, b, ());
        }
        assert!(graph.bipartition().is_none());
    }
}
