//! # Single Source Shortest Path(単一始点最短経路)

use rust_lib_for_comp::graph::adjacency_list::Graph;
use rust_lib_for_comp::graph::dijkstra::Dijkstra;
use rust_lib_for_comp::util::io_util::*;

//#[verify::aizu_online_judge("GRL_1_A")]
pub fn grl_1_a(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let (v, e, r) = reader.v3::<usize, usize, usize>();
    let std = reader.vec3::<usize, usize, i64>(e);
    let mut graph = Graph::new(v);
    for (s, t, d) in std {
        graph.add_arc(s, t, d);
    }
    let d = Dijkstra::calc(&graph, r);
    for i in 0..v {
        if d.dist[i] == i64::MAX {
            writeln!(write, "INF").ok();
        } else {
            writeln!(write, "{}", d.dist[i]).ok();
        }
    }
    write.flush().ok();
}
