#[allow(unused_imports)]
use prim::*;

#[allow(dead_code)]
mod prim {
    use super::*;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    const INF: Weight = 1_000_000_000;

    ///
    /// Prim法でMinimumSpanningTree(最小全域木)を求める
    /// rから開始する (= rと連結でない点は無視する)
    ///
    impl Graph {
        pub fn prim(&self, r: i64) -> Weight {
            let mut t = Edges(Vec::new());
            let mut total: Weight = 0;
            let mut vis = vec![false; self.len()];
            let mut q = BinaryHeap::new();
            q.push(Reverse(Edge::new(-1, r, 0)));
            while !q.is_empty() {
                let Reverse(e) = q.pop().unwrap();
                if vis[e.dst as usize] {
                    continue;
                }
                vis[e.dst as usize] = true;
                total += e.weight;
                if e.src != -1 {
                    t.0.push(e)
                }
                for f in &self.0[e.dst as usize].0 {
                    if !vis[f.dst as usize] {
                        q.push(Reverse(*f));
                    }
                }
            }
            total
        }
    }
}

use crate::libraries::graph::graph::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prim() {
        let data = vec![
            vec![-1, 2, 3, 1, -1],
            vec![2, -1, -1, 4, -1],
            vec![3, -1, -1, 1, 1],
            vec![1, 4, 1, -1, 3],
            vec![-1, -1, 1, 3, -1],
        ];

        let graph = Graph::from_matrix(&data, 5);
        assert_eq!(5, graph.prim(0));
    }
}
