//! # サイクル検出
//! DFSでグラフのサイクルを検出する
//!
//! ## 計算量
//! $O(N)$
//!
//! ## メモ
//! 無向グラフに辺を順番に追加していってサイクルができた瞬間を検出するようなものはunion-findを使う

use super::GraphTrait;
use crate::prelude::*;

#[snippet(name = "find-cycle", doc_hidden)]
pub trait FindCycle {
    /// # サイクル検出
    /// - is_undirected_graph: 無向グラフのときはtrue
    fn find_cycle(&self, is_undirected_graph: bool) -> Option<Vec<usize>>;
    fn dfs(
        &self,
        src: usize,
        used: &mut Vec<u8>,
        pre: &mut Vec<usize>,
        cycle: &mut Vec<usize>,
        is_undirected_graph: bool,
    ) -> bool;
}

#[snippet(name = "find-cycle", doc_hidden)]
impl<G: GraphTrait> FindCycle for G {
    fn find_cycle(&self, is_undirected_graph: bool) -> Option<Vec<usize>> {
        let mut used = vec![0; self.size()];
        let mut pre = vec![self.size(); self.size()];
        let mut cycle = Vec::new();
        for i in 0..self.size() {
            if used[i] == 0 && self.dfs(i, &mut used, &mut pre, &mut cycle, is_undirected_graph) {
                cycle.reverse();
                return Some(cycle);
            }
        }
        None
    }
    fn dfs(
        &self,
        src: usize,
        used: &mut Vec<u8>,
        pre: &mut Vec<usize>,
        cycle: &mut Vec<usize>,
        is_undirected_graph: bool,
    ) -> bool {
        used[src] = 1;
        for (dst, _) in self.edges(src) {
            if is_undirected_graph && pre[src] == dst {
                continue;
            }
            if used[dst] == 0 {
                pre[dst] = src;
                if self.dfs(dst, used, pre, cycle, is_undirected_graph) {
                    return true;
                }
            } else if used[dst] == 1 {
                let mut cur = src;
                while cur != dst {
                    cycle.push(pre[cur]);
                    cur = pre[cur];
                }
                cycle.push(src);
                return true;
            }
        }
        used[src] = 2;
        false
    }
}

#[test]
fn test() {
    use super::adjacency_list::Graph;
    let mut graph = Graph::new(5);
    graph.add_arc(0, 1, ());
    graph.add_arc(1, 2, ());
    graph.add_arc(2, 3, ());
    graph.add_arc(3, 4, ());
    assert_eq!(None, graph.find_cycle(false));
    graph.add_arc(3, 1, ());
    assert_eq!(Some(vec![3, 1, 2]), graph.find_cycle(false));
}

#[test]
fn test2() {
    // 無向グラフの場合は改造してね
    use super::adjacency_list::Graph;
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, ());
    assert_eq!(Some(vec![1, 0]), graph.find_cycle(false));
    assert_eq!(None, graph.find_cycle(true));

    graph.add_edge(1, 2, ());
    graph.add_edge(2, 3, ());
    graph.add_edge(0, 3, ());
    graph.add_edge(3, 4, ());
    assert_eq!(Some(vec![3, 0, 1, 2]), graph.find_cycle(true));
}
