#[allow(dead_code)]
mod lowest_common_ancestor {
    use std::mem::swap;

    /// LowestCommonAncestor(LCA)を求めるライブラリ
    /// 事前処理NlogN、クエリlogN
    pub struct LowestCommonAncestor {
        parent: Vec<Vec<usize>>,
        dist: Vec<usize>,
    }

    impl LowestCommonAncestor {
        pub fn new(graph: &Vec<Vec<usize>>, root: usize) -> LowestCommonAncestor {
            let v = graph.len();
            let mut k = 1;
            while (1 << k) < v {
                k += 1;
            }
            let mut lca = LowestCommonAncestor {
                parent: vec![vec![std::usize::MAX; v]; k],
                dist: vec![std::usize::MAX; v],
            };
            lca.dfs(graph, root, std::usize::MAX, 0);
            for k in 0..k - 1 {
                for v in 0..v {
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
        fn dfs(&mut self, graph: &Vec<Vec<usize>>, v: usize, p: usize, d: usize) {
            self.parent[0][v] = p;
            self.dist[v] = d;
            for &to in &graph[v] {
                if to != p {
                    self.dfs(graph, to, v, d + 1);
                }
            }
        }

        /// u,vの最近共通祖先(LCA)を求める(O(logN))
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
        pub fn get_dist(&mut self, u: usize, v: usize) -> usize {
            let lca = self.query(u, v);
            self.dist[u] + self.dist[v] - 2 * self.dist[lca]
        }

        /// u,vを結ぶパス上に頂点aが存在するかどうか
        pub fn on_path(&mut self, u: usize, v: usize, a: usize) -> bool {
            self.get_dist(u, a) + self.get_dist(a, v) == self.get_dist(u, v)
        }
    }
}

#[allow(unused_imports)]
use lowest_common_ancestor::LowestCommonAncestor;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn it_works() {
        //[0]- 1 - 3 - 6
        // |   |   |
        // 2   4   7
        // |   |
        // 5   8
        let mut graph = vec![Vec::new(); 9];
        graph[0].push(1);
        graph[0].push(2);
        graph[1].push(3);
        graph[1].push(4);
        graph[2].push(5);
        graph[3].push(6);
        graph[3].push(7);
        graph[4].push(8);

        let mut lca = LowestCommonAncestor::new(&graph, 0);

        assert_eq!(0, lca.query(1, 5));
        assert_eq!(2, lca.query(2, 5));
        assert_eq!(1, lca.query(3, 8));
        assert_eq!(3, lca.query(6, 7));

        assert_eq!(2, lca.get_dist(0, 5));
        assert_eq!(4, lca.get_dist(5, 4));

        assert_eq!(true, lca.on_path(5, 8, 1));
        assert_eq!(false, lca.on_path(5, 8, 3));
    }
}
