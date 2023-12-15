//! # ワーシャルフロイド法
//!
//!  ## 計算量
//! 頂点数をNとして $O(N^3)$
//!
use crate::algebra::{BoundedAbove, Zero};
use crate::graph::GraphTrait;
use crate::min_max_macro::{chmin, min};
use crate::prelude::*;

#[codesnip::entry("warshall-floyd")]
pub use warshall_floyd_impl::WarshallFloyd;
#[codesnip::entry("warshall-floyd", include("algebra", "graph", "chmin", "prelude"))]
mod warshall_floyd_impl {
    use super::{chmin, min, Add, BoundedAbove, GraphTrait, Zero};
    pub struct WarshallFloyd<W> {
        dist: Vec<Vec<W>>,
    }

    impl<W: Copy + PartialOrd + BoundedAbove + Add<Output = W> + Zero> WarshallFloyd<W> {
        pub fn build<G: GraphTrait<Weight = W>>(graph: &G) -> Self {
            let mut dist = vec![vec![W::max_value(); graph.size()]; graph.size()];
            dist.iter_mut()
                .enumerate()
                .for_each(|(i, reti)| reti[i] = W::zero());
            (0..graph.size()).for_each(|src| {
                for (dst, weight) in graph.edges(src) {
                    chmin!(dist[src][dst], weight);
                }
            });
            for i in 0..graph.size() {
                for j in 0..graph.size() {
                    for k in 0..graph.size() {
                        if dist[j][i] < W::max_value()
                            && dist[i][k] < W::max_value()
                            && (dist[i][k] >= W::zero()
                                || dist[j][i] >= W::zero()
                                || W::max_value() + dist[j][i] + dist[i][k] >= W::zero())
                        {
                            chmin!(dist[j][k], dist[j][i] + dist[i][k]);
                        }
                    }
                }
            }
            Self { dist }
        }

        pub fn dist(&self, src: usize, dst: usize) -> W {
            self.dist[src][dst]
        }

        /// # 負のサイクル判定
        ///
        /// ## 計算量
        /// $O(N)$
        pub fn contains_negative_cycle(&self) -> bool {
            for (i, dist) in self.dist.iter().enumerate() {
                if dist[i] < W::zero() {
                    return true;
                }
            }
            false
        }
    }
}
