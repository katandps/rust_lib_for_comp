// verification-helper: PROBLEM https://atcoder.jp/contests/abc174/tasks/abc174_f
//! # Static Range Set Query(区間の種類数)
//! クエリを$r$でソートして小さいほうから見る。
//!
//! 各種類について右端の位置を逐次BITで管理することで各クエリについて$O(\logN)$で処理できる。
//!
//! ## 入力
//! - $N$: 数列の個数
//! - $Q$: クエリの回数
//! - $A$: 数列 $0 <= a_i < KINDS$
//! - $LR$: クエリ $[L, R)$
//!
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use addition::Addition;
use binary_indexed_tree::BinaryIndexedTree;
use io_util::*;
use prelude::*;
use range_traits::*;
use string_util::*;

const KINDS: usize = 500_000;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let a = io.vec::<usize>(n);
    let lr = io.vec2::<usize, usize>(q);
    let mut queries = BinaryHeap::new(); // Reverse((r, l, i))
    (0..q).for_each(|i| queries.push(Reverse((lr[i].1, lr[i].0 - 1, i))));
    let mut ans = vec![0; q];
    let mut last_pos = vec![None; KINDS + 1]; // 最後に見つけた位置
    let mut bit = BinaryIndexedTree::<Addition<i64>>::from(n);
    for i in 0..n {
        if let Some(j) = last_pos[a[i]] {
            bit.add(j, -1);
        }
        bit.add(i, 1);
        last_pos[a[i]] = Some(i);

        while let Some(Reverse(query)) = queries.pop() {
            if query.0 > i + 1 {
                queries.push(Reverse(query));
                break;
            }
            ans[query.2] = bit.product(query.1..query.0);
        }
    }
    io.out(ans.join("\n").line());
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4 3
    1 2 1 3
    1 3
    2 4
    3 3",
        "2
        3
        1",
    ));
    solve(io_debug::IODebug::static_assert(
        "10 10
        2 5 6 5 2 1 7 9 7 2
        5 5
        2 4
        6 7
        2 2
        7 8
        7 9
        1 8
        6 9
        8 10
        6 8",
        "1
        2
        2
        1
        2
        2
        6
        3
        3
        3",
    ));
}
