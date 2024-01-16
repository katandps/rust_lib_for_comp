//! # [鉄道路線](https://onlinejudge.u-aizu.ac.jp/problems/0275)

use rust_lib_for_comp::graph::{
    adjacency_list::Graph, dijkstra::Dijkstra, directed_acyclic_graph::Dag,
};
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::util::string_util::*;
use verify::{AizuOnlineJudge, Solver};

#[derive(AizuOnlineJudge)]
pub struct P0275;
impl verify::Solver for P0275 {
    const PROBLEM_ID: &'static str = "0275";
    const TIME_LIMIT_MILLIS: u64 = 8000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (s, r) = reader.v2::<usize, usize>();
        let uvw = reader.vec3::<usize, usize, i64>(r);
        let (a, b, q) = reader.v3::<usize, usize, usize>();
        let cd = reader.vec2::<usize, usize>(q);
        let mut graph = Graph::new(s);
        for &(u, v, w) in &uvw {
            graph.add_edge(u - 1, v - 1, w);
        }
        let pre = Dijkstra::calc(&graph, a - 1);
        let suf = Dijkstra::calc(&graph, b - 1);

        let mut dag = Graph::new(s);
        for &(u, v, w) in &uvw {
            let (u, v) = (u - 1, v - 1);
            if pre.dist[u].wrapping_add(w).wrapping_add(suf.dist[v]) == pre.dist[b - 1] {
                dag.add_arc(u, v, 1);
            }
            if pre.dist[v].wrapping_add(w).wrapping_add(suf.dist[u]) == pre.dist[b - 1] {
                dag.add_arc(v, u, 1);
            }
        }
        let qs: Vec<_> = cd.iter().map(|(c, d)| (c - 1, d - 1)).collect();
        for b in dag.reachability(&qs) {
            writeln!(write, "{}", b.yes()).ok();
        }
        write.flush().ok();
    }
}
