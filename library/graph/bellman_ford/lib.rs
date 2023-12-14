//! # ベルマンフォード法
//! 単一始点の最短経路問題を解く
//! ## 計算量
//! 頂点数$N$, 辺数$M$のとき$O(NM)$
use algebra::{BoundedAbove, BoundedBelow, Zero};
use graph::GraphTrait;
use prelude::*;

macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}

#[codesnip::entry("bellman_ford", doc_hidden)]
pub fn bellman_ford<W, G>(g: &G, src: usize) -> Vec<W>
where
    W: Copy + BoundedAbove + BoundedBelow + Zero + PartialEq + PartialOrd + Add<Output = W>,
    G: GraphTrait<Weight = W>,
{
    let mut dist = vec![W::max_value(); g.size()];
    dist[src] = W::zero();
    for _step1 in 1..g.size() {
        for src in 0..g.size() {
            if dist[src] != W::max_value() {
                g.edges(src).into_iter().for_each(|(dst, weight)| {
                    chmin!(dist[dst], dist[src] + weight);
                });
            }
        }
    }
    let mut neg = vec![false; g.size()];
    for _step2 in 0..g.size() {
        for src in 0..g.size() {
            if dist[src] != W::max_value() {
                g.edges(src).into_iter().for_each(|(dst, weight)| {
                    if chmin!(dist[dst], dist[src] + weight) || neg[src] {
                        neg[dst] = true;
                    }
                });
            }
        }
    }
    dist.into_iter()
        .enumerate()
        .map(|(i, d)| if neg[i] { W::min_value() } else { d })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use adjacency_list::Graph;

    #[test]
    fn test() {
        let n = 10;
        let mut g: Graph<i64> = Graph::new(n);
        g.add_arc(0, 5, 10);
        g.add_arc(5, 8, 10);
        g.add_arc(0, 3, 9);
        g.add_arc(3, 5, 9);
        g.add_arc(3, 8, 9);
        g.add_arc(8, 7, -1);
        g.add_arc(7, 8, -1);

        const INF: i64 = i64::max_value();
        const INF_INV: i64 = i64::min_value();
        let d = bellman_ford(&g, 0);
        assert_eq!(vec![0, INF, INF, 9, INF, 10, INF, INF_INV, INF_INV, INF], d);
    }
}
