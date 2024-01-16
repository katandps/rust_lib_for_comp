//! # Minimum Spanning Tree(最小全域木)

use rust_lib_for_comp::graph::adjacency_list::Graph;
use rust_lib_for_comp::graph::kruskal::Kruskal;
use rust_lib_for_comp::util::io_util::*;

//#[verify::aizu_online_judge("GRL_2_A")]
pub fn grl_2_a(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let (v, e) = reader.v2::<usize, usize>();
    let std = reader.vec3::<usize, usize, i64>(e);
    let mut graph = Graph::new(v);
    for (s, t, d) in std {
        graph.add_edge(s, t, d);
    }
    let d = Kruskal::from(&graph);
    writeln!(write, "{}", d.sum()).ok();
    write.flush().ok();
}
