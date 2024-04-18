use rust_lib_for_comp::{
    algebra::binary_operation::addition::Addition,
    graph::{adjacency_list::Graph, tree::heavy_light_decomposition::HLDecomposition},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct VertexAddPathSum;
impl verify::Solver for VertexAddPathSum {
    const PROBLEM_ID: &'static str = "vertex_add_path_sum";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<i64>(n);
        let mut graph = Graph::new(n);
        for (u, v) in reader.vec2::<usize, usize>(n - 1) {
            graph.add_edge(u, v, ());
        }
        let mut hld = HLDecomposition::<Addition<i64>>::build(&graph, 0, &a, Addition::default());
        for _ in 0..q {
            if 0 == reader.v::<usize>() {
                let (p, x) = reader.v2::<usize, i64>();
                let t = hld.prod_path(p, p);
                hld.update_at(p, t + x);
            } else {
                let (u, v) = reader.v2::<usize, usize>();
                writeln!(write, "{}", hld.prod_path(u, v)).ok();
            }
        }
    }
}
#[test]
fn test() {
    VertexAddPathSum::assert(
        "5 5
        1 10 100 1000 10000
        0 1
        1 2
        2 3
        1 4
        1 0 3
        1 2 4
        0 1 100000
        1 1 3
        1 3 2",
        "1111
        10110
        101110
        1100",
    );
}
