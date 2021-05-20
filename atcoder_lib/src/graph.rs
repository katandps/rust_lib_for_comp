#[allow(unused_imports)]
use graph::*;

#[allow(dead_code)]
pub mod graph {
    use std::cmp::Ordering;
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::fmt::{Debug, Formatter};

    pub type Weight = i64;

    const INF: Weight = 1_000_000_000;

    #[derive(Copy, Clone)]
    pub struct Edge {
        pub src: i64,
        pub dst: i64,
        pub weight: Weight,
    }

    impl Debug for Edge {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} -> {} : {}", self.src, self.dst, self.weight)
        }
    }

    impl Edge {
        pub fn default() -> Edge {
            let (src, dst, weight) = (0, 0, 0);
            Edge { src, dst, weight }
        }

        pub fn new(src: i64, dst: i64, weight: Weight) -> Edge {
            Edge { src, dst, weight }
        }
    }

    impl std::cmp::PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            self.weight.eq(&other.weight)
        }
    }

    impl std::cmp::Eq for Edge {}

    impl std::cmp::PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.weight.partial_cmp(&other.weight)
        }
    }

    impl std::cmp::Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            self.weight.cmp(&other.weight)
        }
    }

    /// 辺の情報を使用してグラフの問題を解くためのライブラリ
    #[derive(Clone, Debug)]
    pub struct Graph(Vec<Vec<Edge>>);

    impl Graph {
        /// n: 頂点数
        pub fn new(n: usize) -> Self {
            Self(vec![Vec::new(); n])
        }

        /// 辺行列からグラフを生成する O(N^2)
        pub fn from_matrix(weights: &Vec<Vec<Weight>>, n: usize) -> Graph {
            let mut ret = Self::new(n);
            for i in 0..n as i64 {
                for j in i + 1..n as i64 {
                    if weights[i as usize][j as usize] == -1 {
                        continue;
                    }
                    ret.add_edge(i, j, weights[i as usize][j as usize]);
                    ret.add_edge(j, i, weights[j as usize][i as usize]);
                }
            }
            ret
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn add_edge(&mut self, a: i64, b: i64, w: Weight) {
            self.0[a as usize].push(Edge::new(a, b, w));
            self.0[b as usize].push(Edge::new(b, a, w));
        }

        pub fn add_arc(graph: &mut Graph, a: i64, b: i64, w: Weight) {
            graph.0[a as usize].push(Edge::new(a, b, w));
        }

        pub fn edges_from(&self, from: usize) -> &Vec<Edge> {
            &self.0[from]
        }

        ///
        /// Prim法でMinimumSpanningTree(最小全域木)を求める
        /// rから開始する (= rと連結でない点は無視する)
        /// ```
        /// use atcoder_lib::graph::graph::Graph;
        /// let data = vec![
        ///     vec![-1, 2, 3, 1, -1],
        ///     vec![2, -1, -1, 4, -1],
        ///     vec![3, -1, -1, 1, 1],
        ///     vec![1, 4, 1, -1, 3],
        ///     vec![-1, -1, 1, 3, -1],
        /// ];
        ///
        /// let graph = Graph::from_matrix(&data, 5);
        /// assert_eq!(5, graph.prim(0));
        /// ```
        ///
        pub fn prim(&self, r: i64) -> Weight {
            let mut t = Vec::new();
            let mut total: Weight = 0;
            let mut visits = vec![false; self.len()];
            let mut q = BinaryHeap::new();
            q.push(Reverse(Edge::new(-1, r, 0)));
            while !q.is_empty() {
                let Reverse(e) = q.pop().unwrap();
                if visits[e.dst as usize] {
                    continue;
                }
                visits[e.dst as usize] = true;
                total += e.weight;
                if e.src != -1 {
                    t.push(e)
                }
                for f in self.edges_from(e.dst as usize).iter() {
                    if !visits[f.dst as usize] {
                        q.push(Reverse(*f));
                    }
                }
            }
            total
        }
    }
}
