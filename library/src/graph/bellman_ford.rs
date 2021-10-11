//! ベルマンフォード法
use crate::algebra::{BoundedAbove, Zero};
use crate::graph::GraphTrait;
use crate::*;

///
///  ベルマンフォード法でlからrへの最小コストを求める
/// ## 計算量
///  O(NM)
pub fn bellman_ford<W, G>(g: &G, l: usize, r: usize) -> W
where
    W: Clone + Copy + BoundedAbove + Zero + PartialEq + PartialOrd + Add<Output = W>,
    G: GraphTrait<Weight = W>,
{
    let mut dist = vec![W::max_value(); g.size()];
    dist[l] = W::zero();
    for _step1 in 1..g.size() {
        for src in 0..g.size() {
            if dist[src] != W::max_value() {
                g.edges(src).iter().for_each(|e| {
                    let _ = chmin!(dist[e.dst], dist[src] + e.weight);
                });
            }
        }
    }
    let mut neg = vec![false; g.size()];
    for _step2 in 0..g.size() {
        for src in 0..g.size() {
            if dist[src] != W::max_value() {
                g.edges(src).iter().for_each(|e| {
                    neg[e.dst] |= neg[src] | chmin!(dist[e.dst], dist[src] + e.weight)
                });
            }
        }
    }
    if neg[r] {
        W::max_value()
    } else {
        dist[r]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test() {
        let n = 10;
        let g: Graph<i64> = Graph::new(n);
        assert_eq!(i64::max_value(), bellman_ford(&g, 0, 9));
    }
}
