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
const KINDS: usize = 500_000;

use rust_lib_for_comp::{
    algebra::binary_operation::addition::Addition,
    data_structure::binary_indexed_tree::BinaryIndexedTree,
    range_traits::RangeProductMut,
    util::{
        io_util::{ReadHelper, ReaderTrait},
        string_util::JoinTrait,
    },
};
use std::{cmp::Reverse, collections::BinaryHeap};
use verify::{AtCoder, Solver};

#[derive(AtCoder)]
pub struct ABC174F;
impl verify::Solver for ABC174F {
    const PROBLEM_ID: &'static str = "abc174_f";
    const TIME_LIMIT_MILLIS: u64 = 1000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let a = reader.vec::<usize>(n);
        let lr = reader.vec2::<usize, usize>(q);
        let mut queries = BinaryHeap::new(); // Reverse((r, l, i))
        (0..q).for_each(|i| queries.push(Reverse((lr[i].1, lr[i].0 - 1, i))));
        let mut ans = vec![0; q];
        let mut last_pos = vec![None; KINDS + 1]; // 最後に見つけた位置
        let mut bit = BinaryIndexedTree::new(n, Addition::default());
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
        writeln!(write, "{}", ans.join("\n")).unwrap()
    }
}

#[test]
fn test() {
    ABC174F::assert(
        "4 3
    1 2 1 3
    1 3
    2 4
    3 3",
        "2
        3
        1",
    );
    ABC174F::assert(
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
    );
}
