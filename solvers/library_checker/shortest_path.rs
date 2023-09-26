//! <https://judge.yosupo.jp/problem/shortest_path>

use adjacency_list::Graph;
use dijkstra::*;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, m, s, t) = io.v4::<usize, usize, usize, usize>();
    let abc = io.vec3::<usize, usize, i64>(m);
    let mut graph = Graph::new(n);
    for (a, b, c) in abc {
        graph.add_arc(a, b, c);
    }
    let dijkstra = Dijkstra::calc(&graph, s);
    if dijkstra.dist[t] == i64::MAX {
        io.out((-1).line());
    } else {
        let path = dijkstra.path(t);
        io.out(format!("{} {}\n", dijkstra.dist[t], path.len() - 1));
        for i in 1..path.len() {
            io.out(format!("{} {}\n", path[i - 1], path[i]));
        }
    }
    io.flush()
}

#[test]
fn test() {
    let io = io_debug::IODebug::new(
        "5 7 2 3
        0 3 5
        0 4 3
        2 4 2
        4 3 10
        4 0 7
        2 1 5
        1 0 1",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new(
                "11 3
                2 1
                1 0
                0 3",
            );
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}

#[test]
fn test2() {
    let io = io_debug::IODebug::new(
        "2 1 0 1
        1 0 10",
        false,
        |outer: &mut ReaderFromStr, _inner: &mut ReaderFromStr| {
            let mut expect = ReaderFromStr::new("-1");
            while let Some(a) = outer.next() {
                assert_eq!(Some(a), expect.next())
            }
            assert_eq!(None, expect.next())
        },
    );
    solve(io);
}
