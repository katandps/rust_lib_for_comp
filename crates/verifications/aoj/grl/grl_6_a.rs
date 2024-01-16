//! # Maximum Flow(最大流)

use rust_lib_for_comp::flow::dinic::*;
use rust_lib_for_comp::util::io_util::*;

//#[verify::aizu_online_judge("GRL_6_A")]
pub fn grl_6_a(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let (v, e) = reader.v2::<usize, usize>();
    let uvc = reader.vec3::<usize, usize, i64>(e);
    let mut dinic = Dinic::new(v);
    for (u, v, c) in uvc {
        dinic.add_edge(u, v, c);
    }
    let ans = dinic.max_flow(0, v - 1);
    writeln!(write, "{ans}").ok();
    write.flush().ok();
}
