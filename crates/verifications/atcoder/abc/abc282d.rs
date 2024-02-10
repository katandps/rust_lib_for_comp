use rust_lib_for_comp::{
    graph::{adjacency_list::Graph, bipartite_graph::BipartiteGraphTrait},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{AtCoder, Solver};

#[derive(AtCoder)]
pub struct ABC282D;
impl verify::Solver for ABC282D {
    const PROBLEM_ID: &'static str = "arc090_d";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m) = reader.v2::<usize, usize>();
        let uv = reader.vec2::<usize, usize>(m);
        let mut graph = Graph::new(n);
        for (u, v) in uv {
            graph.add_edge(u - 1, v - 1, 1);
        }
        let ans = if let Some(v) = graph.bipartition() {
            let mut cnt = vec![vec![0, 0]; n];
            for (c, b) in v {
                cnt[c][usize::from(b)] += 1;
            }
            let ans = cnt.iter().fold(0i64, |x, cnt| {
                x + (cnt[0] + cnt[1]) * (n as i64 - cnt[0] - cnt[1]) + cnt[0] * cnt[1] * 2
            });
            ans / 2 - m as i64
        } else {
            0
        };
        writeln!(write, "{ans}").unwrap()
    }
}

#[test]
fn test() {
    ABC282D::assert(
        "5 4
        4 2
        3 1
        5 2
        3 2",
        "2",
    );
    ABC282D::assert(
        "4 3
        3 1
        3 2
        1 2",
        "0",
    );
    ABC282D::assert(
        "9 11
        4 9
        9 1
        8 2
        8 3
        9 2
        8 4
        6 7
        4 6
        7 5
        4 5
        7 8",
        "9",
    );
}
