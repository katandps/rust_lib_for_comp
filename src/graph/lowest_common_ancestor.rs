//! # 最近共通祖先
//! LowestCommonAncestor(LCA)を求めるライブラリ
//! 事前処理 $`N \log N`$、クエリ$` \log N `$
use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "lowest-common-ancestor", doc_hidden)]
pub struct LowestCommonAncestor<W, G> {
    parent: Vec<Vec<usize>>,
    dist: Vec<usize>,
    _marker: PhantomData<fn() -> (W, G)>,
}

#[snippet(name = "lowest-common-ancestor", doc_hidden)]
impl<W, G> LowestCommonAncestor<W, G>
where
    W: Copy,
    G: GraphTrait<Weight = W>,
{
    pub fn new(g: &G, root: usize) -> Self {
        let mut k = 1;
        while (1 << k) < g.size() {
            k += 1;
        }
        let mut lca = Self {
            parent: vec![vec![std::usize::MAX; g.size()]; k],
            dist: vec![std::usize::MAX; g.size()],
            _marker: Default::default(),
        };
        lca.dfs(g, root, std::usize::MAX, 0);
        for k in 0..k - 1 {
            for v in 0..g.size() {
                if lca.parent[k][v] == std::usize::MAX {
                    lca.parent[k + 1][v] = 1;
                } else {
                    lca.parent[k + 1][v] = lca.parent[k][lca.parent[k][v]];
                }
            }
        }
        lca
    }

    /// graph: グラフ
    /// v: 今見ている頂点
    /// p: parent
    /// d: 根からの距離
    fn dfs(&mut self, g: &G, src: usize, p: usize, d: usize) {
        self.parent[0][src] = p;
        self.dist[src] = d;
        for &to in &g.edges(src) {
            if to.dst != p {
                self.dfs(g, to.dst, src, d + 1);
            }
        }
    }

    /// u,vの最近共通祖先(LCA)を求める$` (O(\log N))`$
    pub fn query(&mut self, mut u: usize, mut v: usize) -> usize {
        // uの深さはvの深さ以上
        if self.dist[u] < self.dist[v] {
            swap(&mut u, &mut v);
        }
        let k = self.parent.len();
        // LCAまでの距離を揃える
        for k in 0..k {
            if (self.dist[u] - self.dist[v]) >> k & 1 == 1 {
                u = self.parent[k][u];
            }
        }
        if u == v {
            u
        } else {
            for i in 0..k {
                let k = k - i - 1;
                if self.parent[k][u] != self.parent[k][v] {
                    u = self.parent[k][u];
                    v = self.parent[k][v];
                }
            }
            self.parent[0][u]
        }
    }

    /// 2頂点u,v間の距離を求める
    pub fn dist(&mut self, u: usize, v: usize) -> usize {
        let lca = self.query(u, v);
        self.dist[u] + self.dist[v] - 2 * self.dist[lca]
    }

    /// u,vを結ぶパス上に頂点aが存在するかどうか
    pub fn on_path(&mut self, u: usize, v: usize, a: usize) -> bool {
        self.dist(u, a) + self.dist(a, v) == self.dist(u, v)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::Graph;

    #[test]
    pub fn it_works() {
        //[0]- 1 - 3 - 6
        // |   |   |
        // 2   4   7
        // |   |
        // 5   8
        let mut graph = Graph::new(9);
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(1, 4, 1);
        graph.add_edge(2, 5, 1);
        graph.add_edge(3, 6, 1);
        graph.add_edge(3, 7, 1);
        graph.add_edge(4, 8, 1);

        let mut lca = LowestCommonAncestor::new(&graph, 0);

        assert_eq!(0, lca.query(1, 5));
        assert_eq!(2, lca.query(2, 5));
        assert_eq!(1, lca.query(3, 8));
        assert_eq!(3, lca.query(6, 7));

        assert_eq!(2, lca.dist(0, 5));
        assert_eq!(4, lca.dist(5, 4));

        assert_eq!(true, lca.on_path(5, 8, 1));
        assert_eq!(false, lca.on_path(5, 8, 3));
    }
}
