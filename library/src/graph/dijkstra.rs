//! ダイクストラ法
use crate::graph::{Graph, Weight, INF};
use crate::*;

pub struct Dijkstra(Vec<Weight>);
impl Dijkstra {
    ///
    /// dijkstra法でlから各頂点への最小コストを求める
    /// 負辺がある場合は使えない
    /// ## 計算量
    ///  O(NlogN)
    pub fn dijkstra(g: &Graph, l: usize) -> Self {
        let mut dist = vec![INF; g.n];
        let mut heap = BinaryHeap::new();
        dist[l] = 0;
        heap.push((Reverse(0), l));
        while let Some((Reverse(d), src)) = heap.pop() {
            if dist[src] != d {
                continue;
            }
            g.edges[src].iter().for_each(|e| {
                if dist[e.dst] > dist[src] + e.weight {
                    dist[e.dst] = dist[src] + e.weight;
                    heap.push((Reverse(dist[e.dst]), e.dst))
                }
            });
        }
        Self(dist)
    }
}
