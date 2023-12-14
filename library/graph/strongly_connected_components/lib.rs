//! 強連結成分分解(SCC)
use adjacency_list::Graph;
use algebra::*;
use graph::GraphTrait;
use prelude::*;

#[codesnip::entry("strongly-connected-components", doc_hidden)]
pub use scc_impl::SCC;
#[codesnip::entry("strongly-connected-components", doc_hidden)]
mod scc_impl {
    use super::*;
    pub struct SCC<W, G> {
        /// もとの頂点と強連結成分の対応
        pub group: Vec<usize>,
        /// 強連結成分をまとめたグラフ(DAG)
        pub graph: Graph<W>,
        /// 強連結成分ごとの個数
        pub size: Vec<usize>,
        /// 強連結成分の個数
        pub n: usize,
        _marker: PhantomData<fn() -> G>,
    }

    impl<W, G> SCC<W, G>
    where
        W: Clone + One,
        G: GraphTrait<Weight = W>,
    {
        /// SCCを構築する O(N + M)
        pub fn build(g: &G) -> Self {
            let mut rest = vec![true; g.size()];
            let mut back_queue = VecDeque::new();
            for src in 0..g.size() {
                if rest[src] {
                    Self::dfs(g, src, &mut back_queue, &mut rest[..]);
                }
            }
            let mut result = vec![None; g.size()];
            let mut i = 0;
            while let Some(src) = back_queue.pop_front() {
                if result[src].is_some() {
                    continue;
                }
                Self::dfs2(g, src, i, &mut result);
                i += 1;
            }

            let mut size = vec![0; g.size()];
            let mut graph = Graph::new(i);
            let mut group = vec![0; g.size()];
            for i in 0..g.size() {
                assert!(result[i].is_some());
                size[result[i].unwrap()] += 1;
                group[i] = result[i].unwrap();
            }

            for src in 0..g.size() {
                for (dst, _weight) in g.edges(src) {
                    let (s, t) = (group[src], group[dst]);
                    if s != t {
                        graph.add_arc(s, t, W::one());
                    }
                }
            }

            SCC {
                group,
                graph,
                size,
                n: i,
                _marker: Default::default(),
            }
        }

        fn dfs(g: &G, src: usize, back_queue: &mut VecDeque<usize>, rest: &mut [bool]) {
            if !rest[src] {
                return;
            }
            rest[src] = false;
            for (dst, _weight) in g.edges(src) {
                Self::dfs(g, dst, back_queue, rest);
            }
            back_queue.push_front(src);
        }

        fn dfs2(g: &G, src: usize, flag: usize, result: &mut Vec<Option<usize>>) {
            if result[src].is_some() {
                return;
            }
            result[src] = Some(flag);
            for (dst, _weight) in g.rev_edges(src) {
                Self::dfs2(g, dst, flag, result);
            }
        }
    }
}
