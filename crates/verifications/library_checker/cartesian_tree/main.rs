// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use cartesian_tree::CartesianTree;
use graph::GraphTrait;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let n = io.v::<usize>();
    let a = io.vec::<usize>(n);
    let ct = CartesianTree::build(&a);
    let mut ans = vec![0; n];
    ans[ct.root] = ct.root;
    for src in 0..n {
        for (dst, _) in ct.graph.edges(src) {
            ans[dst] = src;
        }
    }
    io.out(ans.join(" ").line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3
        1 0 2",
        "1 1 1",
    ));
    solve(io_debug::IODebug::static_assert(
        "11
        9 3 7 1 8 12 10 20 15 18 5",
        "1 3 1 3 10 6 4 8 6 8 3",
    ))
}
