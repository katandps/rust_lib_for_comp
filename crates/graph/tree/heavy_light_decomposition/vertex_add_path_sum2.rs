//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum
use addition::Addition;
use adjacency_list::Graph;
use heavy_light_decomposition::HLDecomposition;
use io_util::*;
use segment_tree::SegmentTree;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let mut graph = Graph::new(n);
    for (u, v) in io.vec2::<usize, usize>(n - 1) {
        graph.add_edge(u, v, ());
    }
    let mut hld =
        HLDecomposition::<SegmentTree<Addition<i64>>>::build_with_weighted_nodes(&graph, 0, &a);

    for _ in 0..q {
        if 0 == io.v() {
            let (p, x) = io.v2::<usize, i64>();
            hld.update_at(p, hld.prod_path(p, p) + x);
        } else {
            let (u, v) = io.v2::<usize, usize>();
            io.out(hld.prod_path(u, v).line());
        }
    }
    io.flush();
}
