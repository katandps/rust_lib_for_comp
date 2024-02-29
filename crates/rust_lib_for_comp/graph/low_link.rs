use super::GraphTrait;
use crate::data_structure::union_find::UnionFind;
pub(crate) use crate::min_max_macro::{chmin, max, min};

#[codesnip::entry("low-link")]
pub use low_link_impl::LowLink;
#[codesnip::entry("low-link", include("graph", "union-find-tree", "chmin", "max"))]
mod low_link_impl {
    use super::{chmin, max, min, GraphTrait, UnionFind};
    #[derive(Clone, Debug)]
    pub struct LowLink {
        ord: Vec<usize>,
        low: Vec<usize>,
        /// # 関節点
        pub articulation: Vec<usize>,
        /// # 橋
        pub bridge: Vec<(usize, usize)>,
    }
    impl LowLink {
        pub fn build<G: GraphTrait>(graph: &G) -> Self {
            let n = graph.size();
            let mut ord = vec![n; n];
            let mut low = vec![n; n];
            let mut articulation = Vec::new();
            let mut bridge = Vec::new();
            let mut uf = UnionFind::new(n);
            Self::dfs(
                0,
                !0,
                0,
                graph,
                &mut uf,
                &mut ord,
                &mut low,
                &mut articulation,
                &mut bridge,
                &mut vec![false; n],
            );
            articulation.sort();
            bridge.sort();
            Self {
                ord,
                low,
                articulation,
                bridge,
            }
        }
        fn dfs<G: GraphTrait>(
            src: usize,
            par: usize,
            mut time: usize,
            graph: &G,
            uf: &mut UnionFind,
            ord: &mut [usize],
            low: &mut [usize],
            articulation: &mut Vec<usize>,
            bridge: &mut Vec<(usize, usize)>,
            done: &mut [bool],
        ) -> usize {
            ord[src] = time;
            time += 1;
            done[src] = true;
            let (mut is_articulation, mut cnt) = (false, 0);
            for (dst, _) in graph.edges(src) {
                if !done[dst] {
                    cnt += 1;
                    time = Self::dfs(
                        dst,
                        src,
                        time,
                        graph,
                        uf,
                        ord,
                        low,
                        articulation,
                        bridge,
                        done,
                    );
                    chmin!(low[src], low[dst]);
                    is_articulation |= (!par > 0) && low[dst] >= ord[src];
                    if ord[src] >= low[dst] {
                        uf.unite(src, dst);
                    } else {
                        bridge.push((min!(src, dst), max!(src, dst)));
                    }
                } else if dst != par {
                    chmin!(low[src], ord[dst]);
                }
            }
            is_articulation |= par == !0 && cnt > 1;
            if is_articulation {
                articulation.push(src)
            }
            time
        }

        pub fn is_bridge(&self, u: usize, v: usize) -> bool {
            self.ord[u] < self.low[v]
        }
    }
}
