//! 2部グラフ
pub mod bipartite_graph {
    /// 与えられたグラフが二部グラフかどうか判定する
    /// 引数は隣接行列
    pub fn is_bipartite_graph(g: &Vec<Vec<bool>>) -> bool {
        let mut dist = vec![None; g.len()];
        dist[0] = Some(true);
        dfs(g.len(), 0, &g, &mut dist)
    }

    fn dfs(n: usize, cur: usize, g: &Vec<Vec<bool>>, d: &mut Vec<Option<bool>>) -> bool {
        for to in 0..n {
            if !g[cur][to] {
                continue;
            }
            if d[to].is_some() && d[cur] == d[to] {
                return false;
            }
            if d[to].is_none() {
                d[to] = Some(!d[cur].unwrap());
                if !dfs(n, to, g, d) {
                    return false;
                }
            }
        }
        true
    }
}
