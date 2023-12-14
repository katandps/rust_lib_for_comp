//! # ダイクストラ法
//!
//! dijkstra法でlから各頂点への最小コストを求める
//! ## 制約
//! 負辺なし
//! ## 計算量
//! O(N \log N)
use algebra::{BoundedAbove, Zero};
use graph::GraphTrait;
use min_max_macro::{chmax, max};
use prelude::*;

#[snippet(name = "dijkstra", doc_hidden)]
pub struct Dijkstra<W> {
    pub dist: Vec<W>,
    prev: Vec<usize>,
    src: usize,
}

#[snippet(name = "dijkstra", doc_hidden)]
impl<W: Copy + BoundedAbove + Add<Output = W> + PartialEq + Ord + Zero> Dijkstra<W> {
    pub fn calc<G: GraphTrait<Weight = W>>(g: &G, src: usize) -> Self {
        let mut dist = vec![W::max_value(); g.size()];
        let mut prev = vec![g.size(); g.size()];
        let mut heap = BinaryHeap::new();
        dist[src] = W::zero();
        heap.push((Reverse(W::zero()), src));
        while let Some((Reverse(d), src)) = heap.pop() {
            if dist[src] != d {
                continue;
            }
            g.edges(src).into_iter().for_each(|(dst, weight)| {
                if dist[dst] > dist[src] + weight {
                    dist[dst] = dist[src] + weight;
                    prev[dst] = src;
                    heap.push((Reverse(dist[dst]), dst))
                }
            });
        }
        Self { dist, prev, src }
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

    pub fn farthest_vertex(&self) -> usize {
        let (mut dist, mut v) = (W::zero(), self.src);
        for (i, d) in self.dist.iter().enumerate() {
            if *d == W::max_value() {
                continue;
            }
            if chmax!(dist, *d) {
                v = i;
            }
        }
        v
    }

    pub fn diameter<G: GraphTrait<Weight = W>>(graph: &G) -> (Self, usize, usize) {
        let d1 = Self::calc(graph, 0);
        let l = d1.farthest_vertex();
        let d2 = Self::calc(graph, l);
        let r = d2.farthest_vertex();
        (d2, l, r)
    }
}
