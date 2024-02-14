//! # Static Range Frequency (整数の出現回数)
//! <https://judge.yosupo.jp/problem/static_range_frequency>
//!
//! ## 入力
//! - $N$: 数列の個数
//! - $Q$: クエリの回数
//! - $A$: 数列
//! - $LRX$: クエリ $[L, R)$ に Xがいくつあるか
//!

use rust_lib_for_comp::{algo::slice_bounds::SliceBounds, util::io_util::*};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct StaticRangeFrequency;
impl verify::Solver for StaticRangeFrequency {
    const PROBLEM_ID: &'static str = "static_range_frequency";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<i64>(n);
        let lrx = reader.vec3::<usize, usize, i64>(q);
        let mut a = a
            .into_iter()
            .enumerate()
            .map(|(i, ai)| (ai, i))
            .collect::<Vec<_>>();
        a.sort_unstable(); // 値が小さいほうから並び、同じ値ならindexが小さいほうが左になるようなソート
        (0..q)
            .map(|i| {
                let (l, r, x) = lrx[i];
                a.lower_bound(&(x, r)) - a.lower_bound(&(x, l))
            })
            .for_each(|ans| {
                writeln!(write, "{ans}").ok();
            });
    }
}
#[test]
fn test() {
    StaticRangeFrequency::assert(
        "5 3
        3 7 1 2 1
        1 5 1
        3 3 0
        0 4 3",
        "2
        0
        1",
    );
}
