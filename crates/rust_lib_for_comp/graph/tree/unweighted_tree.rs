//! # 木
//! グラフが木の場合に使えるアルゴリズム

use crate::graph::GraphTrait;
use crate::min_max_macro::{chmax, max};
use crate::prelude::*;

#[codesnip::entry("tree")]
pub use tree_graph_impl::TreeGraph;

#[codesnip::entry("tree", include("graph", "chmax"))]
mod tree_graph_impl {
    use super::{chmax, max, GraphTrait};
    pub struct TreeGraph;
    impl TreeGraph {
        /// # 頂点の高さ
        /// 適当な頂点を根としたときの頂点の高さを返す
        ///
        /// ## todo
        /// 直径からの高さを求めるべき
        ///
        /// ## verify
        /// [ABC233F](https://atcoder.jp/contests/abc233/submissions/28183153)
        pub fn rank<G: GraphTrait>(graph: &G) -> Vec<i64> {
            let mut rank = vec![None; graph.size()];
            for i in 0..graph.size() {
                if rank[i].is_none() {
                    rank[i] = Some(0);
                    Self::rank_dfs(graph, i, i, &mut rank);
                }
            }
            rank.into_iter().flatten().collect()
        }
        fn rank_dfs<G: GraphTrait>(graph: &G, cur: usize, par: usize, rank: &mut Vec<Option<i64>>) {
            for (dst, _weight) in graph.edges(cur) {
                if dst == par {
                    continue;
                }
                rank[dst] = rank[cur].map(|k| k + 1);
                Self::rank_dfs(graph, dst, cur, rank);
            }
        }

        /// # 最遠点
        /// ## verify
        /// [ABC267F](https://atcoder.jp/contests/abc267/submissions/34679847)
        pub fn farthest_vertex<G: GraphTrait>(graph: &G, src: usize) -> usize {
            let mut rank = vec![None; graph.size()];
            rank[src] = Some(0);
            Self::rank_dfs(graph, src, src, &mut rank);
            let mut dist = -1;
            let mut ret = 0;
            for (i, rank) in rank.iter().enumerate() {
                if let Some(d) = rank {
                    if chmax!(dist, *d) {
                        ret = i;
                    }
                }
            }
            ret
        }

        /// # 直径
        pub fn diameter<G: GraphTrait>(graph: &G) -> (usize, usize) {
            let l = Self::farthest_vertex(graph, 0);
            let r = Self::farthest_vertex(graph, l);
            (l, r)
        }
    }
}
