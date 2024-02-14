use rust_lib_for_comp::{
    algebra::binary_operation::addition::Addition,
    graph::{adjacency_list::Graph, tree::heavy_light_decomposition::HLDecomposition},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct VertexAddSubtreeSum;
impl verify::Solver for VertexAddSubtreeSum {
    const PROBLEM_ID: &'static str = "vertex_add_subtree_sum";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<i64>(n);
        let p = reader.vec::<usize>(n - 1);
        let graph = Graph::tree_root_0(&p);
        let mut hld = HLDecomposition::<Addition<i64>>::build(&graph, 0, &a);
        for _ in 0..q {
            if 0 == reader.v::<usize>() {
                let (u, x) = reader.v2::<usize, i64>();
                hld.update_at(u, hld.prod_path(u, u) + x);
            } else {
                let u = reader.v::<usize>();
                writeln!(write, "{}", hld.prod_subtree(u)).ok();
            }
        }
    }
}
#[test]
fn test() {
    VertexAddSubtreeSum::assert(
        "5 5
        1 10 100 1000 10000
        0 1 2 2
        1 1
        1 2
        0 1 100000
        1 0
        1 3",
        "11110
        11100
        111111
        1000",
    );
}
