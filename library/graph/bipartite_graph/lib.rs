//! # 二部グラフ
//! グラフが二部グラフかどうか判定する
//! 二部グラフだったときはその分割方法を1つ返す
//!
use graph::GraphTrait;
use prelude::*;

#[codesnip::entry("bipartite-graph", doc_hidden)]
pub trait BipartiteGraphTrait: GraphTrait {
    /// # 2部グラフとして分割する
    /// 連結成分ごとに二部グラフとしてbooleanで彩色した結果を返す
    /// 塗り分けられなかった場合はNoneを返す
    fn bipartition(&self) -> Option<Vec<(usize, bool)>> {
        let mut colors = vec![None; self.size()];

        for src in 0..self.size() {
            if colors[src].is_none() {
                colors[src] = Some((src, true));

                let mut ng = false;
                let mut q = VecDeque::from(vec![src]);
                'dfs: while let Some(src) = q.pop_front() {
                    for (dst, _weight) in self.edges(src) {
                        if colors[dst] == colors[src] {
                            ng = true;
                            break 'dfs;
                        }
                        if colors[dst].is_none() {
                            colors[dst] = colors[src].map(|(color, b)| (color, !b));
                            q.push_back(dst);
                        }
                    }
                }
                if ng {
                    return None;
                }
            }
        }
        Some(colors.into_iter().flatten().collect())
    }
}
#[codesnip::entry("bipartite-graph", doc_hidden)]
impl<G: GraphTrait> BipartiteGraphTrait for G {}

#[cfg(test)]
mod test {
    use super::BipartiteGraphTrait;
    use adjacency_list::Graph;

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
