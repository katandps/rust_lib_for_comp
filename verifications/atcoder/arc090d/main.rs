// verification-helper: PROBLEM https://atcoder.jp/contests/arc090/tasks/arc090_b
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use addition::Addition;
use io_util::*;
use string_util::*;
use weighted_union_find::WeightedUnionFind;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, m) = io.v2::<usize, usize>();
    let mut wuf = WeightedUnionFind::<Addition<i64>>::new(n);
    for _ in 0..m {
        let (x, y, d) = io.v3::<usize, usize, i64>();
        wuf.unite(x, y, d);
        if wuf.diff(x, y) != d {
            io.out("No".line());
            return io.flush();
        }
    }
    io.out("Yes".line());
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "3 3
        1 2 1
        2 3 1
        1 3 2",
        "Yes",
    ));
    solve(io_debug::IODebug::static_assert(
        "3 3
        1 2 1
        2 3 1
        1 3 5",
        "No",
    ));
    solve(io_debug::IODebug::static_assert(
        "4 3
        2 1 1
        2 3 5
        3 4 2",
        "Yes",
    ));
    solve(io_debug::IODebug::static_assert(
        "10 3
        8 7 100
        7 9 100
        9 8 100",
        "No",
    ));
    solve(io_debug::IODebug::static_assert("100 0", "Yes"));
}
