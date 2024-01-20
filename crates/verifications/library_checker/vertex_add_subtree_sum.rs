// // verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

// #![cfg_attr(coverage_nightly, feature(coverage_attribute))]
// #[cfg_attr(coverage_nightly, coverage(off))]
// fn main() {
//     solve(io_util::IO::default())
// }
// use addition::Addition;
// use adjacency_list::Graph;
// use heavy_light_decomposition::HLDecomposition;
// use io_util::*;
// use string_util::*;

// pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
//     let (n, q) = io.v2::<usize, usize>();
//     let a = io.vec::<i64>(n);
//     let p = io.vec::<usize>(n - 1);
//     let graph = Graph::tree_root_0(&p);
//     let mut hld = HLDecomposition::<Addition<i64>>::build(&graph, 0, &a);
//     for _ in 0..q {
//         if 0 == io.v() {
//             let (u, x) = io.v2::<usize, i64>();
//             hld.update_at(u, hld.prod_path(u, u) + x);
//         } else {
//             let u = io.v::<usize>();
//             io.out(hld.prod_subtree(u).line());
//         }
//     }
//     io.flush();
// }

// #[test]
// fn test() {
//     solve(io_debug::IODebug::static_assert(
//         "5 5
//         1 10 100 1000 10000
//         0 1 2 2
//         1 1
//         1 2
//         0 1 100000
//         1 0
//         1 3",
//         "11110
//         11100
//         111111
//         1000",
//     ))
// }
