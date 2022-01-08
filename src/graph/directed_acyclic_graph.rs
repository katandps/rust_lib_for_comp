//! # 有向非巡回グラフ(DAG)
//! グラフにサイクルがない場合に使用できるアルゴリズム群

use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "directed-acyclic-graph", doc_hidden)]
pub struct Dag;

#[snippet(name = "directed-acyclic-graph", doc_hidden)]
impl Dag {
    /// # トポロジカルソート
    /// DAGをトポロジカルソートし、結果の頂点列を返す
    ///
    /// ## 計算量
    /// $`O(N)`$
    ///
    /// ## 備考
    /// DAGでない場合はグラフの頂点数と頂点列のサイズが一致しない
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
            g.edges(src).into_iter().for_each(|(dst, _weight)| {
                deg[dst] -= 1;
                if deg[dst] == 0 {
                    q.push_back(dst)
                }
            });
            ret.push(src);
        }
        ret
    }

    /// ## 経路の数え上げ
    /// lを始点とする各点までの経路数をDPで求める
    pub fn path<W, G>(g: &G, l: usize) -> Vec<usize>
    where
        G: GraphTrait<Weight = W>,
    {
        let list = Self::topological_sort(g);
        let mut dp = vec![0; g.size()];
        dp[l] = 1;
        for src in list {
            for (dst, _weight) in g.edges(src) {
                dp[dst] += dp[src];
            }
        }
        dp
    }
}
