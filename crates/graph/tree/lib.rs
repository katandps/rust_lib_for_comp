//! # 木
//! グラフが木の場合に使えるアルゴリズム

use graph::GraphTrait;
use prelude::*;

pub mod heavy_light_decomposition;

#[allow(unused_macros)]
macro_rules! chmax {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_max = max!($($cmps),+);if $base < cmp_max {$base = cmp_max;true} else {false}}};}

#[allow(unused_macros)]
macro_rules! max {($a:expr $(,)*) => {{$a}};($a:expr, $b:expr $(,)*) => {{if $a > $b {$a} else {$b}}};($a:expr, $($rest:expr),+ $(,)*) => {{let b = max!($($rest),+);if $a > b {$a} else {b}}};}

#[snippet(name = "tree-graph", doc_hidden)]
pub trait TreeGraph: GraphTrait {
    /// # 頂点の高さ
    /// 適当な頂点を根としたときの頂点の高さを返す
    ///
    /// ## todo
    /// 直径からの高さを求めるべき
    ///
    /// ## verify
    /// [ABC233F](https://atcoder.jp/contests/abc233/submissions/28183153)
    fn rank(&self) -> Vec<i64> {
        let mut rank = vec![None; self.size()];
        for i in 0..self.size() {
            if rank[i].is_none() {
                rank[i] = Some(0);
                self.rank_dfs(i, i, &mut rank);
            }
        }
        rank.into_iter().flatten().collect()
    }
    fn rank_dfs(&self, cur: usize, par: usize, rank: &mut Vec<Option<i64>>) {
        for (dst, _weight) in self.edges(cur) {
            if dst == par {
                continue;
            }
            rank[dst] = rank[cur].map(|k| k + 1);
            self.rank_dfs(dst, cur, rank);
        }
    }
    /// # いくつかの辺を使って到達できる点のうち、最も遠い点を求める
    ///
    /// ```
    /// use adjacency_list::Graph;
    /// use tree::TreeGraph;
    /// let graph = Graph::<i32>::new(5);
    ///
    /// // l to rが直径
    /// let l = graph.farthest_vertex(0);
    /// let r = graph.farthest_vertex(l);
    /// ```
    ///
    /// ## verify
    /// [ABC267F](https://atcoder.jp/contests/abc267/submissions/34679847)
    fn farthest_vertex(&self, src: usize) -> usize {
        let mut rank = vec![None; self.size()];
        rank[src] = Some(0);
        self.rank_dfs(src, src, &mut rank);
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
}
#[snippet(name = "tree-graph", doc_hidden)]
impl<G: GraphTrait> TreeGraph for G {}
