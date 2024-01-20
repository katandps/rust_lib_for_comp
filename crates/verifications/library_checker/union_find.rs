// // verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind
// #![cfg_attr(coverage_nightly, feature(coverage_attribute))]
// #[cfg_attr(coverage_nightly, coverage(off))]
// fn main() {
//     solve(io_util::IO::default());
// }
// use io_util::*;
// use string_util::*;
// use union_find::UnionFind;

// pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
//     let n = io.v();
//     let mut uf = UnionFind::new(n);
//     for _ in 0..io.v() {
//         let (t, u, v) = io.v3::<usize, usize, usize>();
//         if t == 0 {
//             uf.unite(u, v);
//         } else {
//             io.out(usize::from(uf.same(u, v)).line());
//         }
//     }
//     io.flush();
// }

// #[test]
// fn test() {
//     solve(io_debug::IODebug::static_assert(
//         "4 7
//         1 0 1
//         0 0 1
//         0 2 3
//         1 0 1
//         1 1 2
//         0 0 2
//         1 1 3",
//         "0
//         1
//         0
//         1",
//     ));
// }
