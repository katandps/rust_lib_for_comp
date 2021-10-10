//! 2部グラフ
use super::Graph;

impl<W> Graph<W> {
    /// グラフが二部グラフかどうか判定する
    /// 二部グラフだったときはその分割方法を1つ返す
    /// なお、グラフが連結でない場合は正しく判定できない
    pub fn is_bipartite_graph(&self) -> Option<Vec<bool>> {
        let mut dist = vec![None; self.n];
        dist[0] = Some(true);
        if self.dfs(0, &mut dist) {
            Some(dist.iter().map(|op| op.unwrap()).collect())
        } else {
            None
        }
    }
    fn dfs(&self, src: usize, d: &mut Vec<Option<bool>>) -> bool {
        for edge in &self.edges[src] {
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
