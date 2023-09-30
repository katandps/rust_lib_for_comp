//! # Static Range Frequency (整数の出現回数)
//! <https://judge.yosupo.jp/problem/static_range_frequency>
//!
//! ## 入力
//! - $N$: 数列の個数
//! - $Q$: クエリの回数
//! - $A$: 数列
//! - $LRX$: クエリ $[L, R)$ に Xがいくつあるか
//!

use io_util::*;
use slice_bounds::SliceBounds;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<i64>(n);
    let lrx = io.vec3::<usize, usize, i64>(q);
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
        .for_each(|ans| io.out(ans.line()));
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "5 3
        3 7 1 2 1
        1 5 1
        3 3 0
        0 4 3",
        "2
        0
        1",
    ))
}
