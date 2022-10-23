//! # サイクル検出
//! DFSでグラフのサイクルを検出する
//!
//! ## 計算量
//! $O(N)$
//!
//! ## メモ
//! 無向グラフに辺を順番に追加していってサイクルができた瞬間を検出するようなものはunion-findを使う
//!
//! ## verify
//! [Cycle Detection](https://judge.yosupo.jp/submission/109527)

use super::adjacency_list::Graph;
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
        // 頂点に対する訪問フラグ (0 => 未達, 1 => 到達済み, 2 => 処理済み)
        used: &mut Vec<u8>,
        // 頂点に到達するために通った辺
        pre: &mut Vec<usize>,
        // サイクルをなす辺
        cycle: &mut Vec<usize>,
        // 無向グラフの時 true
        is_undirected_graph: bool,
    ) -> bool;
}

#[snippet(name = "find-cycle", doc_hidden)]
impl<C: Clone> FindCycle for Graph<C> {
    /// サイクルは辺の番号で表現される
    fn find_cycle(&self, is_undirected_graph: bool) -> Option<Vec<usize>> {
        let mut used = vec![0; self.size()];
        let mut pre = vec![self.edges.len(); self.size()];
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
        for next in &self.index[src] {
            let dst = self.edges[*next].1;
            if is_undirected_graph && Some(pre[src]) == self.rev[*next] {
                continue;
            }
            if used[dst] == 0 {
                pre[dst] = *next;
                if self.dfs(dst, used, pre, cycle, is_undirected_graph) {
                    return true;
                }
            } else if used[dst] == 1 {
                let mut cur = src;
                while cur != dst {
                    cycle.push(pre[cur]);
                    cur = self[pre[cur]].0;
                }
                cycle.push(*next);
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
    if let Some(cycle) = graph.find_cycle(false) {
        for i in 0..cycle.len() {
            let (_, dst, ()) = graph[cycle[i]];
            let (src, _, ()) = graph[cycle[(i + 1) % cycle.len()]];
            assert_eq!(dst, src);
        }
    } else {
        assert!(false, "must be found")
    }
}

#[test]
fn test2() {
    use super::adjacency_list::Graph;
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, ());
    assert_eq!(Some(vec![1, 0]), graph.find_cycle(false));
    assert_eq!(None, graph.find_cycle(true));

    graph.add_edge(1, 2, ());
    graph.add_edge(2, 3, ());
    graph.add_edge(0, 3, ());
    graph.add_edge(3, 4, ());
    if let Some(cycle) = graph.find_cycle(false) {
        for i in 0..cycle.len() {
            let (_, dst, ()) = graph[cycle[i]];
            let (src, _, ()) = graph[cycle[(i + 1) % cycle.len()]];
            assert_eq!(dst, src);
        }
    } else {
        assert!(false, "must be found")
    }
}

#[test]
fn undirected_graph() {
    let mut graph = Graph::new(2);
    graph.add_edge(1, 0, ());
    assert!(graph.find_cycle(true).is_none());
}
