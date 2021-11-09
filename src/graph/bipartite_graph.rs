//! # 二部グラフ判定
//! グラフが二部グラフかどうか判定する
//! 二部グラフだったときはその分割方法を1つ返す
//! なお、グラフが連結でない場合は正しく判定できない
use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "judge-bipartite-graph", doc_hidden)]
impl<W> dyn GraphTrait<Weight = W> {
    pub fn is_bipartite_graph(&self) -> Option<Vec<bool>> {
        let mut dist = vec![None; self.size()];
        dist[0] = Some(true);
        if self.dfs(0, &mut dist) {
            Some(dist.iter().map(|op| op.unwrap()).collect())
        } else {
            None
        }
    }
    fn dfs(&self, src: usize, d: &mut Vec<Option<bool>>) -> bool {
        for edge in &self.edges(src) {
            if d[edge.dst] == d[src] {
                return false;
            }
            if d[edge.dst].is_none() {
                d[edge.dst] = d[src].map(|b| !b);
                if !self.dfs(edge.dst, d) {
                    return false;
                }
            }
        }
        true
    }
}
