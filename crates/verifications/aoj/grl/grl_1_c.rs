//! # All Pairs Shortest Path

use rust_lib_for_comp::graph::adjacency_list::Graph;
use rust_lib_for_comp::graph::warshall_floyd::WarshallFloyd;
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::util::string_util::*;

#[verify::aizu_online_judge("GRL_1_C")]
pub fn grl_1_c(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let (v, e) = reader.v2::<usize, usize>();
    let mut graph = Graph::new(v);
    for _ in 0..e {
        let (s, t, d) = reader.v3::<usize, usize, i64>();
        graph.add_arc(s, t, d);
    }
    let wf = WarshallFloyd::build(&graph);
    if wf.contains_negative_cycle() {
        writeln!(write, "NEGATIVE CYCLE").ok();
    } else {
        for i in 0..v {
            writeln!(
                write,
                "{}",
                (0..v)
                    .map(|j| {
                        let d = wf.dist(i, j);
                        if d == i64::MAX {
                            "INF".to_string()
                        } else {
                            d.to_string()
                        }
                    })
                    .join(" "),
            )
            .ok();
        }
    }
    write.flush().ok();
}
