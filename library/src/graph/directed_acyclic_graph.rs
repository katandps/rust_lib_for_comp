//! 有向非巡回グラフ(DAG)

use crate::graph::GraphTrait;
use crate::*;

#[allow(dead_code)]
struct DAG;

#[allow(dead_code)]
impl DAG {
    /// 頂点をトポロジカルソートして返す
    /// グラフがDAGの場合に使用可
    pub fn topological_sort<W, G>(g: &G) -> Vec<usize>
    where
        G: GraphTrait<Weight = W>,
    {
        let mut deg = g.indegree();

        let mut q = VecDeque::new();
        deg.iter().enumerate().for_each(|(i, deg)| {
            if deg == &0 {
                q.push_back(i)
            }
        });

        let mut ret = Vec::new();
        while let Some(src) = q.pop_front() {
            g.edges(src).iter().for_each(|e| {
                deg[e.dst] -= 1;
                if deg[e.dst] == 0 {
                    q.push_back(e.dst)
                }
            });
            ret.push(src);
        }
        ret
    }

    /// lを始点とする各点までの経路数を求める
    /// グラフがDAGの場合に使用可
    pub fn path<W, G>(g: &G, l: usize) -> Vec<usize>
    where
        G: GraphTrait<Weight = W>,
    {
        let list = Self::topological_sort(g);
        let mut dp = vec![0; g.size()];
        dp[l] = 1;
        for src in list {
            for e in &g.edges(src) {
                dp[e.dst] += dp[src];
            }
        }
        dp
    }
}
