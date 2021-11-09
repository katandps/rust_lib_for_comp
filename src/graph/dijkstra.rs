//! # ダイクストラ法
//!
//! dijkstra法でlから各頂点への最小コストを求める
//! ## 制約
//! 負辺なし
//! ## 計算量
//! O(N \log N)
//! ## verify
//! [ARC011C](https://atcoder.jp/contests/arc011/submissions/26722909)
use crate::algebra::{BoundedAbove, Zero};
use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "dijkstra", doc_hidden)]
pub struct Dijkstra<W> {
    dist: Vec<W>,
    prev: Vec<usize>,
}

#[snippet(name = "dijkstra", doc_hidden)]
impl<W: Copy + BoundedAbove + Add<Output = W> + PartialEq + Ord + Zero> Dijkstra<W> {
    pub fn calc<G: GraphTrait<Weight = W>>(g: &G, l: usize) -> Self {
        let mut dist = vec![W::max_value(); g.size()];
        let mut prev = vec![g.size(); g.size()];
        let mut heap = BinaryHeap::new();
        dist[l] = W::zero();
        heap.push((Reverse(W::zero()), l));
        while let Some((Reverse(d), src)) = heap.pop() {
            if dist[src] != d {
                continue;
            }
            g.edges(src).iter().for_each(|e| {
                if dist[e.dst] > dist[src] + e.weight {
                    dist[e.dst] = dist[src] + e.weight;
                    prev[e.dst] = src;
                    heap.push((Reverse(dist[e.dst]), e.dst))
                }
            });
        }
        Self { dist, prev }
    }

    pub fn path(&self, mut to: usize) -> Vec<usize> {
        let mut path = Vec::new();
        while to != self.dist.len() {
            path.push(to);
            to = self.prev[to];
        }
        path.reverse();
        path
    }
}
