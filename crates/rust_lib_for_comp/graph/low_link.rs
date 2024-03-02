//! # LowLink
//! 橋と関節点を求める
//!
//! ## 計算量
//! $O(V)$

use super::GraphTrait;
pub(crate) use crate::min_max_macro::{chmin, min};
use crate::prelude::*;

#[codesnip::entry("low-link")]
pub use low_link_impl::LowLink;
#[codesnip::entry("low-link", include("graph", "prelude", "chmin"))]
mod low_link_impl {
    use super::{chmin, min, swap, GraphTrait};
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
                ord: vec![!0; n],
                low: vec![!0; n],
                articulation: Vec::new(),
                bridge: Vec::new(),
            };
            let mut time = 0;
            for i in 0..n {
                if ret.ord[i] == !0 {
                    time = ret.dfs(i, !0, time, graph);
                }
            }
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
        ) -> usize {
            self.ord[src] = time;
            time += 1;
            let (mut is_articulation, mut cnt) = (false, 0);
            let mut first_p = true; // 多重辺対策
            for (dst, _) in graph.edges(src) {
                if dst == par && first_p {
                    first_p = false;
                    continue;
                }
                if self.ord[dst] == !0 {
                    cnt += 1;
                    time = self.dfs(dst, src, time, graph);
                    chmin!(self.low[src], self.low[dst]);
                    is_articulation |= (!par > 0) && self.low[dst] >= self.ord[src];
                    if self.ord[src] < self.low[dst] {
                        self.bridge.push((src, dst));
                    }
                } else {
                    chmin!(self.low[src], self.ord[dst]);
                }
            }
            is_articulation |= par == !0 && cnt > 1;
            if is_articulation {
                self.articulation.push(src)
            }
            time
        }

        pub fn is_bridge(&self, mut u: usize, mut v: usize) -> bool {
            if self.ord[u] > self.ord[v] {
                swap(&mut u, &mut v);
            }
            self.ord[u] < self.low[v]
        }
    }
}
