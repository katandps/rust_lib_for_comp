// // verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_diameter
// #![cfg_attr(coverage_nightly, feature(coverage_attribute))]
// #[cfg_attr(coverage_nightly, coverage(off))]
// fn main() {
//     solve(io_util::IO::default());
// }
// use adjacency_list::Graph;
// use dijkstra::*;
// use io_util::*;
// use string_util::*;

// pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
//     let n = io.v::<usize>();
//     let abc = io.vec3::<usize, usize, i64>(n - 1);
//     let mut graph = Graph::new(n);
//     for (a, b, c) in abc {
//         graph.add_edge(a, b, c);
//     }
//     let (dist, _l, r) = Dijkstra::diameter(&graph);
//     let path = dist.path(r);
//     io.out(format!("{} {}", dist.dist[r], path.len()).line());
//     io.out(path.join(" ").line());
//     io.flush()
// }

// #[test]
// fn test() {
//     solve(io_debug::IODebug::static_assert(
//         "8
//         0 1 5
//         1 2 3
//         2 3 1
//         1 4 2
//         4 7 4
//         1 5 7
//         2 6 5",
//         "15 4
//         6 2 1 5",
//     ))
// }
