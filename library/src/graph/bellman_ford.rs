//! ベルマンフォード法
use crate::graph::{Graph, Weight, INF};

///
///  ベルマンフォード法でlからrへの最小コストを求める
/// ## 計算量
///  O(NM)
pub fn bellman_ford(g: &Graph, l: usize, r: usize) -> Weight {
    let mut dist = vec![INF; g.n];
    dist[l] = 0;
    for _step1 in 1..g.n {
        for src in 0..g.n {
            if dist[src] != INF {
                g.edges[src].iter().for_each(|e| {
                    let _ = chmin!(dist[e.dst], dist[src] + e.weight);
                });
            }
        }
    }
    let mut neg = vec![false; g.n];
    for _step2 in 0..g.n {
        for src in 0..g.n {
            if dist[src] != INF {
                g.edges[src].iter().for_each(|e| {
                    neg[e.dst] |= neg[src] | chmin!(dist[e.dst], dist[src] + e.weight)
                });
            }
        }
    }
    if neg[r] {
        INF
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
        let g: Graph = Graph::new(n);
        assert_eq!(INF, bellman_ford(&g, 0, 9));
    }
}
