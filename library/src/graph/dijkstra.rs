//! ダイクストラ法
use crate::algebra::{BoundedAbove, Zero};
use crate::graph::GraphTrait;
use crate::*;

///
/// dijkstra法でlから各頂点への最小コストを求める
/// 負辺がある場合は使えない
/// ## 計算量
///  O(NlogN)

pub struct Dijkstra<W>(Vec<W>);
impl<W: Copy + BoundedAbove + Add<Output = W> + PartialEq + Ord + Zero> Dijkstra<W> {
    pub fn dijkstra<G: GraphTrait<Weight = W>>(g: &G, l: usize) -> Self {
        let mut dist = vec![W::max_value(); g.size()];
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
                    heap.push((Reverse(dist[e.dst]), e.dst))
                }
            });
        }
        Self(dist)
    }
}
