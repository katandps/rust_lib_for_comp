//! # 有向非巡回グラフ(DAG)
//! グラフにサイクルがない場合に使用できるアルゴリズム群

use graph::GraphTrait;
use prelude::*;

#[snippet(name = "directed-acyclic-graph", doc_hidden)]
pub trait Dag {
    /// # トポロジカルソート
    /// DAGをトポロジカルソートし、結果の頂点列を返す
    ///
    /// ## 計算量
    /// $O(N)$
    ///
    /// ## 備考
    /// DAGでない場合はグラフの頂点数と頂点列のサイズが一致しない
    fn topological_sort(&self) -> Vec<usize>;

    /// ## 経路の数え上げ
    /// lを始点とする各点までの経路数をDPで求める
    fn path(&self, l: usize) -> Vec<usize>;

    /// ## 到達可能性(オフライン)
    /// 各query: (start, goal)が到達できるかどうかをVecで返す
    /// オフラインで処理することにより、wordsize圧縮ができる
    fn reachability(&self, queries: &[(usize, usize)]) -> Vec<bool>;
}

#[snippet(name = "directed-acyclic-graph", doc_hidden)]
impl<G: GraphTrait> Dag for G {
    fn topological_sort(&self) -> Vec<usize> {
        let mut deg = self.indegree();

        let mut q = VecDeque::new();
        deg.iter().enumerate().for_each(|(i, deg)| {
            if deg == &0 {
                q.push_back(i)
            }
        });

        let mut ret = Vec::new();
        while let Some(src) = q.pop_front() {
            self.edges(src).into_iter().for_each(|(dst, _weight)| {
                deg[dst] -= 1;
                if deg[dst] == 0 {
                    q.push_back(dst)
                }
            });
            ret.push(src);
        }
        ret
    }

    fn path(&self, l: usize) -> Vec<usize> {
        let list = self.topological_sort();
        let mut dp = vec![0; self.size()];
        dp[l] = 1;
        for src in list {
            for (dst, _weight) in self.edges(src) {
                dp[dst] += dp[src];
            }
        }
        dp
    }

    fn reachability(&self, queries: &[(usize, usize)]) -> Vec<bool> {
        let (n, q) = (self.size(), queries.len());
        let order = self.topological_sort();
        let mut ret = vec![false; q];
        let mut l = 0;
        while l < q {
            let r = min(q, l + 64);
            let mut dp = vec![0; n];
            for k in l..r {
                dp[queries[k].0] |= 1 << (k - l);
            }
            for &src in &order {
                for (dst, _) in self.edges(src) {
                    dp[dst] |= dp[src];
                }
            }
            for k in l..r {
                ret[k] = (dp[queries[k].1] >> (k - l)) & 1 > 0;
            }
            l += 64
        }
        ret
    }
}
