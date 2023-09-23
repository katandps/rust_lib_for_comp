//# verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum
use addition::Addition;
use adjacency_list::Graph;
use heavy_light_decomposition::HLDecomposition;
use io_util::*;
use segment_tree::SegmentTree;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
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

#[test]
fn test() {
    let io = io_debug::IODebug::new(
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
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new(
                "1111
                10110
                101110
                1100",
            );
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}
