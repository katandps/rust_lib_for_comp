//! <https://onlinejudge.u-aizu.ac.jp/problems/0275>
use adjacency_list::Graph;
use dijkstra::Dijkstra;
use directed_acyclic_graph::Dag;

pub fn solve(
    s: usize,
    uvw: &[(usize, usize, i64)],
    a: usize,
    b: usize,
    cd: &[(usize, usize)],
) -> Vec<bool> {
    let mut graph = Graph::new(s);
    for &(u, v, w) in uvw {
        graph.add_edge(u - 1, v - 1, w);
    }
    let pre = Dijkstra::calc(&graph, a - 1);
    let suf = Dijkstra::calc(&graph, b - 1);

    let mut dag = Graph::new(s);
    for &(u, v, w) in uvw {
        let (u, v) = (u - 1, v - 1);
        if pre.dist[u].wrapping_add(w).wrapping_add(suf.dist[v]) == pre.dist[b - 1] {
            dag.add_arc(u, v, 1);
        }
        if pre.dist[v].wrapping_add(w).wrapping_add(suf.dist[u]) == pre.dist[b - 1] {
            dag.add_arc(v, u, 1);
        }
    }
    let qs: Vec<_> = cd.iter().map(|(c, d)| (c - 1, d - 1)).collect();
    dag.reachability(&qs)
}

#[test]
fn test() {
    use io_debug::*;
    use io_util::*;
    use string_util::*;

    let mut io = IODebug::new(
        "6 7
        1 2 3
        1 4 1
        2 3 5
        4 3 1
        3 6 2
        4 5 2
        5 6 1
        1 6 6
        1 6
        4 3
        4 6
        5 6
        2 6
        2 5",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new(
                "Yes
                Yes
                Yes
                Yes
                No
                No",
            );
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    );
    let (s, r) = io.v2::<usize, usize>();
    let uvw = io.vec3::<usize, usize, i64>(r);
    let (a, b, q) = io.v3::<usize, usize, usize>();
    let cd = io.vec2::<usize, usize>(q);
    let bools = solve(s, &uvw, a, b, &cd);
    for b in bools {
        io.out(if b { "Yes" } else { "No" }.line());
    }
    io.flush()
}