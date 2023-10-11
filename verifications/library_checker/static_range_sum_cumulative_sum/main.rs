// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use addition::Addition;
use cumulative_sum::CumulativeSum;
use io_util::*;
use range_traits::RangeProduct;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let cm = a.into_iter().collect::<CumulativeSum<Addition<i64>>>();
    for _ in 0..q {
        let (l, r) = io.v2::<usize, usize>();
        io.out(cm.product(l..r).line());
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 5
        1 10 100 1000 10000
        2 3
        0 3
        2 5
        3 4
        0 5",
        "100
        111
        11100
        1000
        11111",
    ))
}
