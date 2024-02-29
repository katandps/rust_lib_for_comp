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
            let mut ret = Self {
                ord: vec![n; n],
                low: vec![n; n],
                articulation: Vec::new(),
                bridge: Vec::new(),
            };
            let mut uf = UnionFind::new(n);
            ret.dfs(0, !0, 0, graph, &mut uf, &mut vec![false; n]);
            ret.articulation.sort();
            ret.bridge.sort();
            ret
        }
        fn dfs<G: GraphTrait>(
            &mut self,
            src: usize,
            par: usize,
            mut time: usize,
            graph: &G,
            uf: &mut UnionFind,
            done: &mut [bool],
        ) -> usize {
            self.ord[src] = time;
            time += 1;
            done[src] = true;
            let (mut is_articulation, mut cnt) = (false, 0);
            for (dst, _) in graph.edges(src) {
                if !done[dst] {
                    cnt += 1;
                    time = self.dfs(dst, src, time, graph, uf, done);
                    chmin!(self.low[src], self.low[dst]);
                    is_articulation |= (!par > 0) && self.low[dst] >= self.ord[src];
                    if self.ord[src] >= self.low[dst] {
                        uf.unite(src, dst);
                    } else {
                        self.bridge.push((min!(src, dst), max!(src, dst)));
                    }
                } else if dst != par {
                    chmin!(self.low[src], self.ord[dst]);
                }
            }
            is_articulation |= par == !0 && cnt > 1;
            if is_articulation {
                self.articulation.push(src)
            }
            time
        }

        pub fn is_bridge(&self, u: usize, v: usize) -> bool {
            self.ord[u] < self.low[v]
        }
    }
}
