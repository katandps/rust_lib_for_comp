use rust_lib_for_comp::{
    algebra::mod_int::ModInt,
    element::affine::{Affine, Composition},
    graph::{adjacency_list::Graph, tree::heavy_light_decomposition::HLDecomposition},
    util::io_util::{ReadHelper, ReaderTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct VertexSetPathComposite;
impl verify::Solver for VertexSetPathComposite {
    const PROBLEM_ID: &'static str = "vertex_set_path_composite";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let af = reader
            .vec2::<ModInt, ModInt>(n)
            .into_iter()
            .map(|(a, b)| Affine::new(a, b))
            .collect::<Vec<_>>();
        let mut graph = Graph::new(n);
        for (u, v) in reader.vec2::<usize, usize>(n - 1) {
            graph.add_edge(u, v, ());
        }
        let mut hld = HLDecomposition::<Composition<ModInt<998_244_353>>>::build(&graph, 0, &af);
        for _ in 0..q {
            if 0 == reader.v::<usize>() {
                let (p, c, d) = reader.v3::<usize, ModInt, ModInt>();
                hld.update_at(p, Affine::new(c, d));
            } else {
                let (u, v, x) = reader.v3::<usize, usize, ModInt>();
                let af = hld.prod_path(u, v);
                writeln!(write, "{}", af.apply(x)).ok();
            }
        }
    }
}
#[test]
fn test() {
    VertexSetPathComposite::assert(
        "1 1
    100000 100000
    1 0 0 100000",
        "17656470",
    );
    VertexSetPathComposite::assert(
        "5 5
        1 2
        3 4
        5 6
        7 8
        9 10
        0 1
        1 2
        2 3
        2 4
        1 0 3 11
        1 2 4 12
        0 2 13 14
        1 0 4 15
        1 2 2 16",
        "1555
        604
        6571
        222",
    );
    VertexSetPathComposite::assert(
        "7 7
        1 2
        2 3
        3 4
        4 5
        5 6
        6 7
        7 8
        0 1
        1 2
        0 3
        3 4
        0 5
        5 6
        1 2 4 1
        1 4 6 1
        1 6 2 1
        0 1 20 30
        1 2 4 1
        1 4 6 1
        1 6 2 1",
        "411
        2199
        607
        3471
        2199
        6034",
    );
}
