//! # 木
//! グラフが木の場合に使えるアルゴリズム

pub mod cartesian_tree;
pub mod euler_tour;
pub mod heavy_light_decomposition;
pub mod lowest_common_ancestor;

use crate::algebra::{BoundedBelow, Zero};
use crate::graph::GraphTrait;
use crate::min_max_macro::{chmax, max};
use crate::prelude::*;

#[codesnip::entry("tree")]
pub use tree_graph_impl::TreeGraph;

#[codesnip::entry("tree", include("graph", "chmax", "algebra"))]
mod tree_graph_impl {
    use super::{chmax, max, Add, BoundedBelow, GraphTrait, Zero};
    pub trait TreeGraph<G: GraphTrait> {
        fn rank(&self) -> Vec<G::Weight>;
        fn rank_dfs(&self, cur: usize, par: usize, rank: &mut Vec<Option<G::Weight>>);
        fn farthest_vertex(&self, src: usize) -> (usize, G::Weight);
        fn diameter(&self) -> (usize, usize, G::Weight);
    }
    impl<G: GraphTrait> TreeGraph<G> for G
    where
        G::Weight: Add<Output = G::Weight> + Zero + Clone + BoundedBelow + PartialOrd,
    {
        /// # 頂点の高さ
        /// 適当な頂点を根としたときの頂点の高さを返す
        ///
        /// ## todo
        /// 直径からの高さを求めるべき
        ///
        /// ## verify
        /// [ABC233F](https://atcoder.jp/contests/abc233/submissions/28183153)
        fn rank(&self) -> Vec<G::Weight> {
            let mut rank = vec![None; self.size()];
            for i in 0..self.size() {
                if rank[i].is_none() {
                    rank[i] = Some(G::Weight::zero());
                    self.rank_dfs(i, i, &mut rank);
                }
            }
            rank.into_iter().flatten().collect()
        }
        fn rank_dfs(&self, cur: usize, par: usize, rank: &mut Vec<Option<G::Weight>>) {
            for (dst, weight) in self.edges(cur) {
                if dst == par {
                    continue;
                }
                rank[dst] = rank[cur].clone().map(|k| k + weight);
                self.rank_dfs(dst, cur, rank);
            }
        }

        /// # 最遠点 とその点までの 距離
        /// ## verify
        /// [ABC267F](https://atcoder.jp/contests/abc267/submissions/34679847)
        fn farthest_vertex(&self, src: usize) -> (usize, G::Weight) {
            let mut rank = vec![None; self.size()];
            rank[src] = Some(G::Weight::zero());
            self.rank_dfs(src, src, &mut rank);
            let mut dist = G::Weight::min_value();
            let mut ret = 0;
            for (i, rank) in rank.iter().enumerate() {
                if let Some(d) = rank {
                    if chmax!(dist, d.clone()) {
                        ret = i;
                    }
                }
            }
            (ret, dist)
        }

        /// # 直径
        fn diameter(&self) -> (usize, usize, G::Weight) {
            let (l, _) = self.farthest_vertex(0);
            let (r, d) = self.farthest_vertex(l);
            (l, r, d)
        }
    }
}
