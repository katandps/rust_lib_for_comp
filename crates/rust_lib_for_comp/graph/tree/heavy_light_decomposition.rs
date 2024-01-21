//! # HL分解
//! 値が変化する木上のパスクエリ/部分木クエリの計算量を改善する
//!
//! ## todo
//! rootが0でないときにバグあり
use crate::algebra::*;
use crate::data_structure::segment_tree::SegmentTree;
use crate::graph::GraphTrait;
use crate::prelude::*;
use crate::range_traits::*;

#[codesnip::entry("heavy-light-decomposition")]
pub use heavy_light_decomposition_impl::HLDecomposition;
#[codesnip::entry(
    "heavy-light-decomposition",
    include("prelude", "graph", "algebra", "range-traits", "segment-tree")
)]
mod heavy_light_decomposition_impl {
    use super::{swap, GraphTrait, Monoid, PointUpdate, Range, RangeProduct, SegmentTree};

    #[derive(Clone, Debug)]
    pub struct HLDecomposition<M: Monoid, const WEIGHTED_EDGE: bool = false> {
        /// 木の頂点数
        n: usize,
        /// 木の根
        root: usize,
        /// その頂点を根とする部分木の頂点数
        size: Vec<usize>,
        /// 行きがけ順で頂点に到達した時間
        in_time: Vec<usize>,
        /// 初めて到達した時間から頂点への逆引き
        /// これを使って区間データを初期化するとよい
        rev: Vec<usize>,
        /// 行きがけ順で頂点から抜けた時間
        /// 部分木クエリに利用する
        out_time: Vec<usize>,
        /// 同じ連結成分で最も根に近い頂点
        head: Vec<usize>,
        /// 頂点の直接の親
        parent: Vec<usize>,
        /// 根から頂点までの距離
        depth: Vec<usize>,
        /// 累積データ構造
        upward: SegmentTree<M>,
        downward: SegmentTree<M, true>,
    }

    impl<M: Monoid, const WEIGHTED_EDGE: bool> HLDecomposition<M, WEIGHTED_EDGE> {
        /// # 頂点に重さが設定されている木を初期化する
        pub fn build<G: GraphTrait>(g: &G, root: usize, weights: &[M::M]) -> Self {
            let mut this = Self {
                n: g.size(),
                root,
                size: vec![1; g.size()],
                in_time: vec![0; g.size()],
                rev: vec![0; g.size()],
                out_time: vec![0; g.size()],
                head: vec![0; g.size()],
                parent: vec![root; g.size()],
                depth: vec![0; g.size()],
                upward: SegmentTree::from(Vec::new()),
                downward: SegmentTree::from(Vec::new()),
            };
            let max_childs = this.dfs_size(g);
            this.dfs_hld(g, max_childs);
            if !WEIGHTED_EDGE {
                let src = this
                    .rev
                    .iter()
                    .map(|i| weights[*i].clone())
                    .collect::<Vec<_>>();
                this.upward = SegmentTree::from(src.clone());
                this.downward = SegmentTree::from(src);
            } else {
                // todo 辺に重みがある場合
            }
            this
        }

        /// # 辺に重さが設定されている木を初期化する
        pub fn build_with_weighted_edges<G>(g: &G, root: usize) -> Self
        where
            G: GraphTrait<Weight = M::M>,
        {
            let mut this = Self {
                n: g.size(),
                root,
                size: vec![1; g.size()],
                in_time: vec![!0; g.size()],
                rev: vec![0; g.size()],
                out_time: vec![0; g.size()],
                head: vec![0; g.size()],
                parent: vec![root; g.size()],
                depth: vec![0; g.size()],
                upward: SegmentTree::from(Vec::new()),
                downward: SegmentTree::from(Vec::new()),
            };
            let max_childs = this.dfs_size(g);
            this.dfs_hld(g, max_childs);
            this
        }

        /// # 辺に設定された重みをweightに変更する
        pub fn update_edge(&mut self, u: usize, v: usize, weight: M::M) {
            assert!(WEIGHTED_EDGE);
            let p = std::cmp::min(u, v);

            self.update_at(p, weight);
            unimplemented!();
        }

        /// # 頂点に設定された重みをweightに変更する
        pub fn update_at(&mut self, p: usize, weight: M::M) {
            self.upward.update_at(self.in_time[p], weight.clone());
            self.downward.update_at(self.in_time[p], weight);
        }

        /// # Pathの値の総和
        pub fn prod_path(&self, mut u: usize, mut v: usize) -> M::M {
            let mut swapping = false;
            // front:u側 back:v側
            let (mut front, mut back) = (M::unit(), M::unit());
            while self.head[u] != self.head[v] {
                if self.in_time[self.head[u]] > self.in_time[self.head[v]] {
                    swap(&mut u, &mut v);
                    swapping ^= true;
                }
                // v側を一つ上の列にシフトする
                if swapping {
                    back = M::op(
                        &back,
                        &self
                            .downward
                            .product(self.in_time[self.head[v]]..=self.in_time[v]),
                    );
                } else {
                    front = M::op(
                        &self
                            .upward
                            .product(self.in_time[self.head[v]]..=self.in_time[v]),
                        &front,
                    );
                }
                v = self.parent[self.head[v]];
            }
            if self.in_time[u] > self.in_time[v] {
                swap(&mut u, &mut v);
                swapping ^= true;
            }

            let (l, r) = (
                self.in_time[u] + usize::from(WEIGHTED_EDGE),
                self.in_time[v],
            );
            if swapping {
                M::op(&back, &M::op(&self.downward.product(l..=r), &front))
            } else {
                M::op(&back, &M::op(&self.upward.product(l..=r), &front))
            }
        }

        /// # rを根とする部分木の値の総和
        pub fn prod_subtree(&self, r: usize) -> M::M {
            self.upward.product(self.subtree_to_range(r))
        }

        /// 部分木のサイズを求めつつ、直接の子のうち、部分木のサイズが最も大きいもののリストを返す
        /// srcの子で部分木のサイズが一番大きいものがgraph[src][0]に来るようにする
        fn dfs_size<G: GraphTrait>(&mut self, g: &G) -> Vec<usize> {
            let mut dfs = vec![self.root];
            let mut max_childs = vec![!0; g.size()];
            while let Some(src) = dfs.pop() {
                if src < self.n {
                    dfs.push(!src);
                    for (dst, _w) in g.edges(src) {
                        if dst == self.parent[src] {
                            continue;
                        }
                        self.parent[dst] = src;
                        self.depth[dst] = self.depth[src] + 1;
                        dfs.push(dst);
                    }
                } else {
                    let (mut max_child, mut max_child_size) = (!0, 0);
                    for (dst, _w) in g.edges(!src) {
                        if dst == self.parent[!src] {
                            continue;
                        }
                        self.size[!src] += self.size[dst];
                        if max_child_size < self.size[dst] {
                            max_child_size = self.size[dst];
                            max_child = dst;
                        }
                    }
                    max_childs[!src] = max_child;
                }
            }
            max_childs
        }

        fn dfs_hld<G: GraphTrait>(&mut self, g: &G, max_childs: Vec<usize>) {
            let mut times = 0;
            let mut dfs = vec![self.root];
            while let Some(src) = dfs.pop() {
                if src < self.n {
                    self.in_time[src] = times;
                    times += 1;
                    self.rev[self.in_time[src]] = src;
                    dfs.push(!src);
                    // 0番目を先に探索したい -> 最後にやる
                    for (dst, _w) in g.edges(src) {
                        if dst == self.parent[src] || max_childs[src] == dst {
                            continue;
                        }
                        self.head[dst] = dst;
                        dfs.push(dst);
                    }

                    if max_childs[src] < self.n {
                        self.head[max_childs[src]] = self.head[src];
                        dfs.push(max_childs[src]);
                    }
                } else {
                    self.out_time[!src] = times;
                }
            }
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

        /// # vを根とする部分木を区間に変換する
        pub fn subtree_to_range(&self, v: usize) -> Range<usize> {
            self.in_time[v]..self.out_time[v]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::algebra::binary_operation::addition::Addition;
    use crate::algo::xor_shift::XorShift;
    use crate::element::sequence::Sequence;
    use crate::graph::adjacency_list::Graph;

    #[test]
    fn random_sequence_test() {
        let mut random = XorShift::default();
        for _ in 0..100 {
            let n = 100;
            let mut graph = Graph::new(n);
            for i in 1..n {
                let p = random.rand(i as u64);
                graph.add_edge(p as usize, i, ());
            }
            let v = (0..n).map(Sequence::new).collect::<Vec<_>>();
            let root = 0; //random.rand(n as u64) as usize;
            let hld = HLDecomposition::<Addition<Sequence<usize>>>::build(&graph, root, &v);
            for l in 0..100 {
                for r in 0..100 {
                    let mut result = Vec::new();
                    dfs(&graph, l, r, &mut Vec::new(), &mut result);
                    assert_eq!(
                        Sequence(result),
                        hld.prod_path(l, r),
                        "{} {}\n{:?}\n{:?}",
                        l,
                        r,
                        &graph,
                        hld
                    );
                }
            }
        }
    }

    fn dfs(
        graph: &Graph<()>,
        src: usize,
        goal: usize,
        buf: &mut Vec<usize>,
        result: &mut Vec<usize>,
    ) {
        buf.push(src);
        if goal == src {
            *result = buf.clone();
            buf.pop();
            return;
        }

        for (dst, _) in graph.edges(src) {
            if buf.len() > 1 && buf[buf.len() - 2] == dst {
                continue;
            }
            dfs(graph, dst, goal, buf, result);
        }
        buf.pop();
    }

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

        let hld = HLDecomposition::<Addition<i64>, true>::build(&graph, 0, &Vec::new());

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
        graph.add_edge(0, 1, ());
        graph.add_edge(0, 2, ());
        graph.add_edge(2, 3, ());
        graph.add_edge(2, 4, ());
        graph.add_edge(4, 6, ());
        graph.add_edge(5, 6, ());

        let hld = HLDecomposition::<Addition<i64>>::build(&graph, 0, &vec![1, 2, 4, 8, 16, 32, 64]);
        assert_eq!(
            vec![127, 2, 124, 8, 112, 32, 96],
            (0..7).map(|i| hld.prod_subtree(i)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_path() {
        //
        // 0 - 2 - 4 - 5
        // |   |
        // 1   3
        // |   |
        // 6   7
        //
        let n = 8;
        let mut graph = Graph::new(n);
        graph.add_edge(0, 1, ());
        graph.add_edge(0, 2, ());
        graph.add_edge(2, 3, ());
        graph.add_edge(2, 4, ());
        graph.add_edge(4, 5, ());
        graph.add_edge(1, 6, ());
        graph.add_edge(3, 7, ());

        let mut hld = HLDecomposition::<Addition<Sequence<usize>>>::build(
            &graph,
            0,
            &(0..n).map(|i| Sequence::new(i)).collect::<Vec<_>>()[..],
        );
        assert_eq!(Sequence(vec![4, 2, 0, 1]), hld.prod_path(4, 1));
        assert_eq!(Sequence(vec![1, 0, 2, 4]), hld.prod_path(1, 4));

        assert_eq!(Sequence(vec![0]), hld.prod_path(0, 0));
        assert_eq!(Sequence(vec![0, 1]), hld.prod_path(0, 1));
        assert_eq!(Sequence(vec![0, 2]), hld.prod_path(0, 2));
        assert_eq!(Sequence(vec![0, 2, 3]), hld.prod_path(0, 3));
        assert_eq!(Sequence(vec![1, 0, 2]), hld.prod_path(1, 2));
        assert_eq!(Sequence(vec![2, 4]), hld.prod_path(2, 4));
        assert_eq!(Sequence(vec![4, 2]), hld.prod_path(4, 2));
        assert_eq!(Sequence(vec![1, 0, 2, 3]), hld.prod_path(1, 3));

        assert_eq!(Sequence(vec![6, 1, 0, 2, 4, 5]), hld.prod_path(6, 5));
        assert_eq!(Sequence(vec![5, 4, 2, 0, 1, 6]), hld.prod_path(5, 6));
        assert_eq!(Sequence(vec![6, 1, 0, 2, 3, 7]), hld.prod_path(6, 7));
        assert_eq!(Sequence(vec![7, 3, 2, 0, 1, 6]), hld.prod_path(7, 6));
        assert_eq!(Sequence(vec![5, 4, 2, 3, 7]), hld.prod_path(5, 7));
        assert_eq!(Sequence(vec![7, 3, 2, 4, 5]), hld.prod_path(7, 5));

        hld.update_at(0, Sequence::new(100));
        assert_eq!(Sequence(vec![7, 3, 2, 100, 1, 6]), hld.prod_path(7, 6));
        assert_eq!(Sequence(vec![6, 1, 100, 2, 3, 7]), hld.prod_path(6, 7));

        hld.update_at(7, Sequence::new(700));
        assert_eq!(Sequence(vec![700, 3, 2, 100, 1, 6]), hld.prod_path(7, 6));
        assert_eq!(Sequence(vec![6, 1, 100, 2, 3, 700]), hld.prod_path(6, 7));

        hld.update_at(2, Sequence::new(200));
        assert_eq!(Sequence(vec![700, 3, 200, 100, 1, 6]), hld.prod_path(7, 6));
        assert_eq!(Sequence(vec![6, 1, 100, 200, 3, 700]), hld.prod_path(6, 7));

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
        let hld = HLDecomposition::<Addition<Sequence<i32>>>::build(
            &graph,
            0,
            &(0..5).map(|i| vec![i]).map(Sequence).collect::<Vec<_>>()[..],
        );
        assert_eq!(Sequence(vec![0, 1, 2, 3]), hld.prod_path(0, 3));
        assert_eq!(Sequence(vec![2, 1, 4]), hld.prod_path(2, 4));
        assert_eq!(Sequence(vec![1, 2, 3]), hld.prod_path(1, 3));
        assert_eq!(Sequence(vec![2]), hld.prod_path(2, 2));
    }
}
