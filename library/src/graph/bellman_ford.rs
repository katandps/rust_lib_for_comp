//! ベルマンフォード法
use crate::algebra::{BoundedAbove, Zero};
use crate::graph::Graph;
use crate::*;

///
///  ベルマンフォード法でlからrへの最小コストを求める
/// ## 計算量
///  O(NM)
pub fn bellman_ford<
    W: Clone + Copy + BoundedAbove + Zero + PartialEq + PartialOrd + Add<Output = W>,
>(
    g: &Graph<W>,
    l: usize,
    r: usize,
) -> W {
    let mut dist = vec![W::max_value(); g.n];
    dist[l] = W::zero();
    for _step1 in 1..g.n {
        for src in 0..g.n {
            if dist[src] != W::max_value() {
                g.edges[src].iter().for_each(|e| {
                    let _ = chmin!(dist[e.dst], dist[src] + e.weight);
                });
            }
        }
    }
    let mut neg = vec![false; g.n];
    for _step2 in 0..g.n {
        for src in 0..g.n {
            if dist[src] != W::max_value() {
                g.edges[src].iter().for_each(|e| {
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

    #[test]
    fn test() {
        let n = 10;
        let g: Graph<i64> = Graph::new(n);
        assert_eq!(i64::max_value(), bellman_ford(&g, 0, 9));
    }
}
