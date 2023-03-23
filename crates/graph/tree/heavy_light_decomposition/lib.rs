//! # HL分解
//! 値が変化する木上のパスクエリ/部分木クエリの計算量を改善する
//!
//! ## todo
//! 非可換なモノイドについてもパスクエリが実行できるようにする
use algebra::*;
use graph::GraphTrait;
use prelude::*;
use range_traits::*;

#[snippet(name = "heavy-light-decomposition", doc_hidden)]
pub use heavy_light_decomposition_impl::HLDecomposition;
#[snippet(name = "heavy-light-decomposition", doc_hidden)]
mod heavy_light_decomposition_impl {
    use super::{swap, GraphTrait, Magma, PointUpdate, Range, RangeProduct, ToBounds};

    #[derive(Clone, Debug)]
    pub struct HLDecomposition<D> {
        graph: Vec<Vec<usize>>,
        _root: usize,
        size: Vec<usize>,
        /// 行きがけ順で頂点に到達した時間
        in_time: Vec<usize>,
        /// 初めて到達した時間から頂点への逆引き
        /// これを使って区間データを初期化するとよい
        rev: Vec<usize>,
        /// 行きがけ順で頂点から抜けた時間
        out_time: Vec<usize>,
        /// 同じ連結成分で最も根に近い頂点
        head: Vec<usize>,
        /// 頂点の直接の親
        parent: Vec<usize>,
        /// 根から頂点までの距離
        depth: Vec<usize>,
        /// 辺コストを頂点に移しているか
        edge: bool,
        upward: D,
    }

    impl<D> HLDecomposition<D> {
        /// # 頂点に重さが設定されている木を初期化する
        pub fn build_with_weighted_nodes<G, W>(g: &G, root: usize, weights: &[W]) -> Self
        where
            G: GraphTrait,
            D: From<Vec<W>>,
            W: Clone,
        {
            let mut this = Self {
                graph: vec![Vec::new(); g.size()],
                _root: root,
                size: vec![1; g.size()],
                in_time: vec![0; g.size()],
                rev: vec![0; g.size() * 2],
                out_time: vec![0; g.size()],
                head: vec![0; g.size()],
                parent: vec![root; g.size()],
                depth: vec![0; g.size()],
                edge: false,
                upward: D::from(Vec::new()),
            };
            this.build_graph(g, root, root);
            this.dfs_size(root, root);
            this.dfs_hld(root, root, &mut 0);
            let src = this
                .rev
                .iter()
                .map(|i| weights[*i].clone())
                .collect::<Vec<_>>();
            this.upward = D::from(src);
            this
        }

        /// # 辺に重さが設定されている木を初期化する
        pub fn build_with_weighted_edges<G, W>(g: &G, root: usize) -> Self
        where
            G: GraphTrait<Weight = W>,
            D: From<Vec<W>>,
        {
            let mut this = Self {
                graph: vec![Vec::new(); g.size()],
                _root: root,
                size: vec![1; g.size()],
                in_time: vec![0; g.size()],
                rev: vec![0; g.size() * 2],
                out_time: vec![0; g.size()],
                head: vec![0; g.size()],
                parent: vec![root; g.size()],
                depth: vec![0; g.size()],
                edge: true,
                upward: D::from(Vec::new()),
            };
            this.build_graph(g, root, root);
            this.dfs_size(root, root);
            this.dfs_hld(root, root, &mut 0);
            this
        }

        /// # 辺に設定された重みをweightに変更する
        pub fn update_edge<W: Clone>(&mut self, u: usize, v: usize, weight: W)
        where
            D: PointUpdate<W>,
        {
            assert!(self.edge);
            for r in self.path_to_ranges(u, v) {
                let (l, r) = r.lr();
                if r > l {
                    self.upward.update_at(l, weight.clone());
                }
            }
        }

        /// # 頂点に設定された重みをweightに変更する
        pub fn update_at<W: Clone>(&mut self, p: usize, weight: W)
        where
            D: PointUpdate<W>,
        {
            self.upward.update_at(self.in_time[p], weight);
        }

        pub fn prod_path(&self, u: usize, v: usize) -> <D::Magma as Magma>::M
        where
            D: RangeProduct<usize>,
        {
            let vec = self
                .path_to_ranges(u, v)
                .into_iter()
                .map(|r| self.upward.product(r))
                .collect::<Vec<_>>();
            assert!(!vec.is_empty());
            let mut ret = vec[0].clone();
            for item in vec.iter().skip(1) {
                ret = D::Magma::op(&ret, item)
            }
            ret
        }

        fn build_graph<G: GraphTrait>(&mut self, g: &G, src: usize, par: usize) {
            for (dst, _w) in g.edges(src) {
                if dst == par {
                    continue;
                }
                self.graph[src].push(dst);
                self.build_graph(g, dst, src);
            }
        }

        /// 部分木のサイズを求めつつ、
        /// srcの子で部分木のサイズが一番大きいものがgraph[src]に来るようにする
        fn dfs_size(&mut self, src: usize, par: usize) {
            self.parent[src] = par;
            for dst_i in 0..self.graph[src].len() {
                let dst = self.graph[src][dst_i];
                if dst == par {
                    continue;
                }
                self.depth[dst] = self.depth[src] + 1;
                self.dfs_size(dst, src);
                self.size[src] += self.size[dst];
                if self.size[dst] > self.size[self.graph[src][0]] {
                    self.graph[src].swap(0, dst_i);
                }
            }
        }

        fn dfs_hld(&mut self, src: usize, par: usize, times: &mut usize) {
            self.in_time[src] = *times;
            self.rev[self.in_time[src]] = src;
            *times += 1;

            for dst in self.graph[src].clone() {
                if dst == par || dst == src {
                    continue;
                }
                // graph[src][0] == dst <=> src->dstがheavy-path
                self.head[dst] = if self.graph[src][0] == dst {
                    self.head[src]
                } else {
                    dst
                };
                self.dfs_hld(dst, src, times);
            }
            self.out_time[src] = *times;
        }

        /// # 頂点vからrootの方向にk個さかのぼった頂点を返す
        pub fn la(&self, mut v: usize, mut k: usize) -> usize {
            loop {
                let u = self.head[v];
                if self.in_time[v] - k >= self.in_time[u] {
                    return self.rev[self.in_time[v] - k];
                }
                k -= self.in_time[v] - self.in_time[u] + 1;
                v = self.parent[u];
            }
        }

        /// # 最近共通祖先: Lowest Common Ancestor
        pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
            loop {
                if self.in_time[u] > self.in_time[v] {
                    swap(&mut u, &mut v);
                }
                if self.head[u] == self.head[v] {
                    return u;
                }
                v = self.parent[self.head[v]];
            }
        }

        /// # 2点間の距離
        pub fn dist(&self, u: usize, v: usize) -> usize {
            self.depth[u] + self.depth[v] - 2 * self.depth[self.lca(u, v)]
        }

        /// # 頂点uから頂点vへのパスを区間の集合に変換する
        pub fn path_to_ranges(&self, mut u: usize, mut v: usize) -> Vec<Range<usize>> {
            let mut ret = Vec::new();
            while self.head[u] != self.head[v] {
                if self.in_time[self.head[u]] > self.in_time[self.head[v]] {
                    swap(&mut u, &mut v);
                }
                ret.push(self.in_time[self.head[v]]..self.in_time[v] + 1);
                v = self.parent[self.head[v]];
            }
            if self.in_time[u] > self.in_time[v] {
                swap(&mut u, &mut v)
            }
            if self.in_time[u] + usize::from(self.edge) < self.in_time[v] + 1 {
                ret.push(self.in_time[u] + usize::from(self.edge)..self.in_time[v] + 1);
            }
            ret
        }

        /// # vを根とする部分木を区間に変換する
        pub fn subtree_to_range(&self, v: usize) -> Range<usize> {
            self.in_time[v]..self.out_time[v]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use addition::Addition;
    use adjacency_list::Graph;
    use binary_indexed_tree::BinaryIndexedTree;
    use segment_tree::SegmentTree;
    use sequence::Sequence;

    #[test]
    fn test() {
        //
        // 0 - 2 - 4
        // |   |
        // 1   3
        //
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(2, 4, 1);

        let hld = HLDecomposition::<BinaryIndexedTree<Addition<i64>>>::build_with_weighted_edges(
            &graph, 0,
        );

        {
            assert_eq!(0, hld.la(0, 0));

            assert_eq!(1, hld.la(1, 0));
            assert_eq!(0, hld.la(1, 1));

            assert_eq!(2, hld.la(2, 0));
            assert_eq!(0, hld.la(2, 1));

            assert_eq!(3, hld.la(3, 0));
            assert_eq!(2, hld.la(3, 1));
            assert_eq!(0, hld.la(3, 2));

            assert_eq!(4, hld.la(4, 0));
            assert_eq!(2, hld.la(4, 1));
            assert_eq!(0, hld.la(4, 2));
        }

        {
            assert_eq!(0, hld.lca(0, 0));
            assert_eq!(0, hld.lca(0, 1));
            assert_eq!(0, hld.lca(0, 2));
            assert_eq!(0, hld.lca(0, 3));
            assert_eq!(0, hld.lca(0, 4));

            assert_eq!(1, hld.lca(1, 1));
            assert_eq!(0, hld.lca(1, 2));
            assert_eq!(0, hld.lca(1, 3));
            assert_eq!(0, hld.lca(1, 4));

            assert_eq!(2, hld.lca(2, 2));
            assert_eq!(2, hld.lca(2, 3));
            assert_eq!(2, hld.lca(2, 4));

            assert_eq!(3, hld.lca(3, 3));
            assert_eq!(2, hld.lca(3, 4));

            assert_eq!(4, hld.lca(4, 4));
        }

        //
        // 0 - 2 - 4 - 6 - 5
        // |   |
        // 1   3
        //
        let mut graph = Graph::new(7);
        graph.add_edge(0, 1, 1);
        graph.add_edge(0, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(2, 4, 1);
        graph.add_edge(4, 6, 1);
        graph.add_edge(5, 6, 1);

        let hld = HLDecomposition::<BinaryIndexedTree<Addition<i64>>>::build_with_weighted_edges(
            &graph, 0,
        );

        dbg!(hld.path_to_ranges(0, 5));
        dbg!(hld.path_to_ranges(1, 3));

        for i in 0..7 {
            dbg!(hld.subtree_to_range(i));
        }
    }

    #[test]
    fn test_path() {
        //
        // 0 - 2 - 4
        // |   |
        // 1   3
        //
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1, ());
        graph.add_edge(0, 2, ());
        graph.add_edge(2, 3, ());
        graph.add_edge(2, 4, ());

        let hld =
            HLDecomposition::<SegmentTree<Addition<Sequence<i32>>>>::build_with_weighted_nodes(
                &graph,
                0,
                &(0..5).map(|i| vec![i]).map(Sequence).collect::<Vec<_>>()[..],
            );
        dbg!(hld.prod_path(0, 1));
        dbg!(hld.prod_path(1, 2));
        dbg!(hld.prod_path(4, 2));
        dbg!(hld.prod_path(1, 3));
        dbg!(hld.prod_path(4, 1));

        //
        // 0 - 1 - 2 - 3
        //     |
        //     4
        //
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1, ());
        graph.add_edge(1, 2, ());
        graph.add_edge(2, 3, ());
        graph.add_edge(1, 4, ());
        let hld =
            HLDecomposition::<SegmentTree<Addition<Sequence<i32>>>>::build_with_weighted_nodes(
                &graph,
                0,
                &(0..5).map(|i| vec![i]).map(Sequence).collect::<Vec<_>>()[..],
            );
        dbg!(hld.prod_path(0, 3));
        dbg!(hld.prod_path(2, 4));
        dbg!(hld.prod_path(1, 3));
        dbg!(hld.prod_path(3, 2));
    }
}
