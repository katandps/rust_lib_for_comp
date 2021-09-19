//! 強連結成分分解(SCC)
use super::Graph;

#[allow(unused_imports)]
use strongly_connected_components::SCC;

#[allow(dead_code)]
pub mod strongly_connected_components {
    use super::Graph;
    use std::collections::{HashSet, VecDeque};

    pub struct SCC {
        /// もとの頂点と強連結成分の対応
        pub group: Vec<usize>,
        /// 強連結成分をまとめたグラフ(DAG)
        pub graph: Graph,
        /// 強連結成分ごとの個数
        pub size: Vec<usize>,
        /// 強連結成分の個数
        pub n: usize,
    }

    impl SCC {
        /// SCCを構築する O(N + M)
        pub fn build(g: &Graph) -> SCC {
            let mut rest = (0..g.n).collect::<HashSet<_>>();
            let mut back_queue = VecDeque::new();
            while let Some(&src) = rest.iter().next() {
                Self::dfs(&g, src, &mut back_queue, &mut rest);
            }
            let mut result = vec![None; g.n];
            let mut i = 0;
            while let Some(src) = back_queue.pop_front() {
                if result[src].is_some() {
                    continue;
                }
                Self::dfs2(&g, src, i, &mut result);
                i += 1;
            }

            let mut size = vec![0; g.n];
            let mut graph = Graph::new(i);
            let mut group = vec![0; g.n];
            for i in 0..g.n {
                assert!(result[i].is_some());
                size[result[i].unwrap()] += 1;
                group[i] = result[i].unwrap();
            }

            for i in 0..g.n {
                for edge in &g.edges[i] {
                    let (s, t) = (group[edge.src], group[edge.dst]);
                    if s != t {
                        graph.add_arc(s, t, 1);
                    }
                }
            }

            SCC {
                group,
                graph,
                size,
                n: i,
            }
        }

        fn dfs(g: &Graph, src: usize, back_queue: &mut VecDeque<usize>, rest: &mut HashSet<usize>) {
            if !rest.contains(&src) {
                return;
            }
            rest.remove(&src);
            for edge in &g.edges[src] {
                Self::dfs(g, edge.dst, back_queue, rest);
            }
            back_queue.push_front(src);
        }

        fn dfs2(g: &Graph, src: usize, flag: usize, result: &mut Vec<Option<usize>>) {
            if result[src].is_some() {
                return;
            }
            result[src] = Some(flag);
            for edge in &g.rev_edges[src] {
                Self::dfs2(g, edge.dst, flag, result);
            }
        }
    }
}
