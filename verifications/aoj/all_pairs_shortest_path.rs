// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_C
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(IO::default());
}
use complib::graph::adjacency_list::Graph;
use complib::graph::warshall_floyd::WarshallFloyd;
use complib::util::io_util::*;
use complib::util::string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (v, e) = io.v2::<usize, usize>();
    let mut graph = Graph::new(v);
    for _ in 0..e {
        let (s, t, d) = io.v3::<usize, usize, i64>();
        graph.add_arc(s, t, d);
    }
    let wf = WarshallFloyd::build(&graph);
    if wf.contains_negative_cycle() {
        io.out("NEGATIVE CYCLE".line())
    } else {
        for i in 0..v {
            io.out(
                (0..v)
                    .map(|j| {
                        let d = wf.dist(i, j);
                        if d == i64::MAX {
                            "INF".to_string()
                        } else {
                            d.to_string()
                        }
                    })
                    .join(" ")
                    .line(),
            );
        }
    }
    io.flush();
}

#[verify::aizu_online_judge("GRL_1_C")]
pub fn grl_1_c(reader: impl std::io::Read, mut writer: impl std::io::Write) {}
#[test]
fn test() {
    solve(complib::util::io_debug::IODebug::static_assert(
        "4 6
        0 1 1
        0 2 5
        1 2 2
        1 3 4
        2 3 1
        3 2 7",
        "0 1 3 4
        INF 0 2 3
        INF INF 0 1
        INF INF 7 0",
    ));
    solve(complib::util::io_debug::IODebug::static_assert(
        "4 6
        0 1 1
        0 2 -5
        1 2 2
        1 3 4
        2 3 1
        3 2 7",
        "0 1 -5 -4
        INF 0 2 3
        INF INF 0 1
        INF INF 7 0",
    ));
    solve(complib::util::io_debug::IODebug::static_assert(
        "4 6
        0 1 1
        0 2 5
        1 2 2
        1 3 4
        2 3 1
        3 2 -7",
        "NEGATIVE CYCLE",
    ));
}
