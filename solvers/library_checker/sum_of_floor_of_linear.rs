//! <https://judge.yosupo.jp/problem/sum_of_floor_of_linear>

use floor_sum::floor_sum;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    for _ in 0..io.v::<usize>() {
        let (n, m, a, b) = io.v4::<i64, i64, i64, i64>();
        io.out(floor_sum(n, m, a, b).line());
    }
    io.flush();
}
#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5
        4 10 6 3
        6 5 4 3
        1 1 0 0
        31415 92653 58979 32384
        1000000000 1000000000 999999999 999999999",
        "3
        13
        0
        314095480
        499999999500000000",
    ))
}
