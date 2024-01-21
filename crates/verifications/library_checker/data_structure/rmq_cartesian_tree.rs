// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use cartesian_tree::CartesianTree;
use io_util::*;
use lowest_common_ancestor::LowestCommonAncestor;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let ct = CartesianTree::build(&a);
    let lca = LowestCommonAncestor::new(&ct.graph, ct.root);
    for _ in 0..q {
        let (l, r) = io.v2::<usize, usize>();
        io.out(a[lca.query(l, r - 1)].line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4 10
        2 10 1 100
        0 1
        0 2
        0 3
        0 4
        1 2
        1 3
        1 4
        2 3
        2 4
        3 4",
        "2
        2
        1
        1
        10
        1
        1
        1
        1
        100",
    ))
}
