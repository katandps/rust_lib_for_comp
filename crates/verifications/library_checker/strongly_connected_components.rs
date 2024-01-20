// // verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc
// #![cfg_attr(coverage_nightly, feature(coverage_attribute))]
// #[cfg_attr(coverage_nightly, coverage(off))]
// fn main() {
//     solve(io_util::IO::default());
// }
// use adjacency_list::Graph;
// use directed_acyclic_graph::Dag;
// use io_util::*;
// use string_util::*;
// use strongly_connected_components::SCC;

// pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
//     let (n, m) = io.v2::<usize, usize>();
//     let ab = io.vec2::<usize, usize>(m);
//     let mut graph = Graph::new(n);
//     for (a, b) in ab {
//         graph.add_arc(a, b, 1);
//     }
//     let scc = SCC::build(&graph);
//     scc.graph.topological_sort();
//     io.out(scc.n.line());
//     let mut ans = vec![Vec::new(); scc.n];
//     for i in 0..n {
//         ans[scc.group[i]].push(i);
//     }
//     for v in ans {
//         io.out(format!("{} {}\n", v.len(), v.join(" ")));
//     }
//     io.flush();
// }

// #[test]
// fn test() {
//     solve(io_debug::IODebug::static_assert(
//         "6 7
//         1 4
//         5 2
//         3 0
//         5 5
//         4 1
//         0 3
//         4 2",
//         "4
//         1 5
//         2 1 4
//         1 2
//         2 0 3",
//     ))
// }
