//! # ダイクストラ法
//!
use crate::algebra::{BoundedAbove, Zero};
use crate::graph::GraphTrait;
use crate::*;

///
/// dijkstra法でlから各頂点への最小コストを求める
/// ## 制約
/// 負辺なし
/// ## 計算量
/// O(NlogN)
/// ## verify
/// [ARC011C](https://atcoder.jp/contests/arc011/submissions/26722909)
pub struct Dijkstra<W> {
    dist: Vec<W>,
    prev: Vec<usize>,
}
impl<W> Dijkstra<W>
where
    W: Copy + BoundedAbove + Add<Output = W> + PartialEq + Ord + Zero,
{
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
