// verification-helper: PROBLEM https://atcoder.jp/contests/abc147/tasks/abc147_f
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use fxhasher::HashMap;
use io_util::*;
use string_util::*;
use union_of_ranges::unite;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, mut x, mut d) = io.v3::<i64, i64, i64>();
    if d == 0 {
        return io.out(if x == 0 { 1 } else { n + 1 }.line());
    }
    if d < 0 {
        x = -x;
        d = -d;
    }

    let mut map = HashMap::default();
    for i in 0..=n {
        map.entry((x * i).rem_euclid(d)).or_insert(Vec::new()).push(
            (x * i).div_euclid(d) + i * (i - 1) / 2
                ..=(x * i).div_euclid(d) + n * (n - 1) / 2 - (n - i) * (n - i - 1) / 2,
        );
    }
    let mut ans = 0;
    for (_, v) in map {
        let v = unite(&v);
        for r in v {
            ans += r.count();
        }
    }
    io.out(ans.line());
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert("3 4 2", "8"));
    solve(io_debug::IODebug::static_assert("2 3 -3", "2"));
    solve(io_debug::IODebug::static_assert("100 14 20", "49805"));
}
