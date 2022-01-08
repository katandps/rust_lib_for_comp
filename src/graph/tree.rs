//! # 木

use crate::graph::GraphTrait;
use crate::prelude::*;

/// # 頂点の高さ
/// 適当な頂点を根としたときの頂点の高さを返す
///
/// ## todo
/// 直径からの高さを求めるべき
///
/// ## verify
/// [ABC233F](https://atcoder.jp/contests/abc233/submissions/28183153)
#[snippet(name = "tree-graph", doc_hidden)]
pub fn rank<G: GraphTrait>(g: &G) -> Vec<i64> {
    let mut rank = vec![None; g.size()];
    for i in 0..g.size() {
        if rank[i].is_none() {
            rank[i] = Some(0);
            rank_dfs(i, i, g, &mut rank);
        }
    }
    rank.into_iter().flatten().collect()
}

#[snippet(name = "tree-graph", doc_hidden)]
fn rank_dfs<G: GraphTrait>(cur: usize, par: usize, g: &G, rank: &mut Vec<Option<i64>>) {
    for (dst, _weight) in g.edges(cur) {
        if dst == par {
            continue;
        }
        rank[dst] = rank[cur].map(|k| k + 1);
        rank_dfs(dst, cur, g, rank);
    }
}
