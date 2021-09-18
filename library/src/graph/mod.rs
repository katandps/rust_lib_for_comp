//! グラフライブラリ

// ここから
#[allow(unused_imports)]
use graph::*;

#[allow(unused_macros)]
macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
#[allow(unused_macros)]
macro_rules! chmax {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_max = max!($($cmps),+);if $base < cmp_max {$base = cmp_max;true} else {false}}};}
#[allow(unused_macros)]
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}
#[allow(unused_macros)]
macro_rules! max {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$a} else {$b}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = max!($($rest),+);if $a > b {$a} else {b}}};
}

pub mod bipartite_graph;
pub mod grid;
pub mod lowest_common_ancestor;
pub mod union_find;
pub mod warshall_floyd;

#[allow(dead_code)]
pub mod graph {
    use std::cmp::Ordering;
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, VecDeque};
    use std::fmt::{Debug, Formatter};

    pub type Weight = i64;

    pub const INF: Weight = 1 << 60;

    #[derive(Copy, Clone)]
    pub struct Edge {
        pub src: usize,
        pub dst: usize,
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

        pub fn new(src: usize, dst: usize, weight: Weight) -> Edge {
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
    pub struct Graph {
        n: usize,
        edges: Vec<Vec<Edge>>,
        rev_edges: Vec<Vec<Edge>>,
    }

    impl Graph {
        /// n: 頂点数
        pub fn new(n: usize) -> Self {
            Self {
                n,
                edges: vec![Vec::new(); n],
                rev_edges: vec![Vec::new(); n],
            }
        }

        /// 辺行列からグラフを生成する O(N^2)
        pub fn from_matrix(weights: &Vec<Vec<Weight>>, n: usize) -> Graph {
            let mut ret = Self::new(n);
            for i in 0..n {
                for j in i + 1..n {
                    if weights[i][j] == -1 {
                        continue;
                    }
                    ret.add_edge(i, j, weights[i as usize][j as usize]);
                    ret.add_edge(j, i, weights[j as usize][i as usize]);
                }
            }
            ret
        }

        /// 相互に行き来できる辺をつける
        pub fn add_edge(&mut self, a: usize, b: usize, w: Weight) {
            self.edges[a].push(Edge::new(a, b, w));
            self.edges[b].push(Edge::new(b, a, w));
            self.rev_edges[a].push(Edge::new(a, b, w));
            self.rev_edges[b].push(Edge::new(b, a, w));
        }

        /// 1方向にのみ移動できる辺をつける
        pub fn add_arc(&mut self, a: usize, b: usize, w: Weight) {
            self.edges[a].push(Edge::new(a, b, w));
            self.rev_edges[b].push(Edge::new(b, a, w));
        }

        ///
        /// Prim法でMinimumSpanningTree(最小全域木)を求める
        /// rから開始する (= rと連結でない点は無視する)
        /// ## 計算量
        /// 頂点数をV、辺数をEとすると
        /// 二分ヒープによる実装なのでO(ElogV)
        /// ```
        /// use library::graph::graph::Graph;
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
        pub fn prim(&self, r: usize) -> Weight {
            let mut t = Vec::new();
            let mut total: Weight = 0;
            let mut visits = vec![false; self.n];
            let mut q = BinaryHeap::new();
            q.push(Reverse(Edge::new(self.n, r, 0)));
            while !q.is_empty() {
                let Reverse(e) = q.pop().unwrap();
                if visits[e.dst as usize] {
                    continue;
                }
                visits[e.dst as usize] = true;
                total += e.weight;
                if e.src != self.n {
                    t.push(e)
                }
                self.edges[e.dst].iter().for_each(|f| {
                    if !visits[f.dst as usize] {
                        q.push(Reverse(*f));
                    }
                });
            }
            total
        }

        ///
        ///  ベルマンフォード法でlからrへの最小コストを求める
        /// ## 計算量
        ///  O(NM)
        pub fn bellman_ford(&self, l: usize, r: usize) -> Weight {
            let mut dist = vec![INF; self.n];
            dist[l] = 0;
            for _step1 in 1..self.n {
                for src in 0..self.n {
                    if dist[src] != INF {
                        self.edges[src].iter().for_each(|e| {
                            let _ = chmin!(dist[e.dst], dist[src] + e.weight);
                        });
                    }
                }
            }
            let mut neg = vec![false; self.n];
            for _step2 in 0..self.n {
                for src in 0..self.n {
                    if dist[src] != INF {
                        self.edges[src].iter().for_each(|e| {
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

        ///
        /// dijkstra法でlから各頂点への最小コストを求める
        /// 負辺がある場合は使えない
        /// ## 計算量
        ///  O(NlogN)
        pub fn dijkstra(&self, l: usize) -> Vec<Weight> {
            let mut dist = vec![INF; self.n];
            let mut heap = BinaryHeap::new();
            dist[l] = 0;
            heap.push((Reverse(0), l));
            while let Some((Reverse(d), src)) = heap.pop() {
                if dist[src] != d {
                    continue;
                }
                self.edges[src].iter().for_each(|e| {
                    if dist[e.dst] > dist[src] + e.weight {
                        dist[e.dst] = dist[src] + e.weight;
                        heap.push((Reverse(dist[e.dst]), e.dst))
                    }
                });
            }
            dist
        }

        /// 各頂点の入次数を返す
        fn indegree(&self) -> Vec<i32> {
            (0..self.n)
                .map(|dst| self.rev_edges[dst].len() as i32)
                .collect()
        }

        /// 各頂点の出次数を返す
        fn outdegree(&self) -> Vec<i32> {
            (0..self.n)
                .map(|src| self.edges[src].len() as i32)
                .collect()
        }

        /// 頂点をトポロジカルソートして返す
        /// グラフがDAGの場合に使用可
        pub fn topological_sort(&self) -> Vec<usize> {
            let mut deg = self.indegree();

            let mut q = VecDeque::new();
            for i in 0..self.n {
                if deg[i] == 0 {
                    q.push_back(i);
                }
            }

            let mut ret = Vec::new();
            while let Some(src) = q.pop_front() {
                self.edges[src].iter().for_each(|e| {
                    deg[e.dst] -= 1;
                    if deg[e.dst] == 0 {
                        q.push_back(e.dst)
                    }
                });
                ret.push(src);
            }
            ret
        }

        /// lを始点とする各点までの経路数を求める
        /// グラフがDAGの場合に使用可
        pub fn path(&self, l: usize) -> Vec<usize> {
            let list = self.topological_sort();
            let mut dp = vec![0; self.n];
            dp[l] = 1;
            for src in list {
                for e in &self.edges[src] {
                    dp[e.dst] += dp[src];
                }
            }
            dp
        }

        /// 後退解析で各点をスタートとしたときの勝敗を求める
        /// 0: 未定/引き分け
        /// 1: 勝ち
        /// 2: 負け
        pub fn retrograde_analysis(&self) -> Vec<usize> {
            #[derive(Clone, Copy, Debug, PartialEq)]
            enum Res {
                DRAW,
                WIN,
                LOSE,
            }
            let mut deg = self.outdegree();
            let mut ret = vec![Res::DRAW; self.n];

            let mut q = VecDeque::new();
            for i in 0..self.n {
                if deg[i] == 0 {
                    ret[i] = Res::LOSE;
                    q.push_back(i);
                }
            }
            while let Some(src) = q.pop_front() {
                self.rev_edges[src].iter().for_each(|e| {
                    if ret[e.dst] == Res::DRAW {
                        deg[e.dst] -= 1;
                        if ret[src] == Res::LOSE {
                            ret[e.dst] = Res::WIN;
                            q.push_back(e.dst);
                        } else if deg[e.dst] == 0 {
                            ret[e.dst] = Res::LOSE;
                            q.push_back(e.dst);
                        }
                    }
                });
            }
            ret.into_iter().map(|r| r as usize).collect()
        }
    }
}

// ここまで

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let n = 10;
        let g = Graph::new(n);
        assert_eq!(graph::INF, g.bellman_ford(0, 9));
    }
}
