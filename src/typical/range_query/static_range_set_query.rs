//! # Static Range Set Query(区間の種類数)
//! クエリを$`r`$でソートして小さいほうから見る。
//!
//! 各種類について右端の位置を逐次BITで管理することで各クエリについて$`O(\logN)`$で処理できる。
//!
//! ## 入力
//! - $`N`$: 数列の個数
//! - $`Q`$: クエリの回数
//! - $`A`$: 数列 $`0 <= a_i < KINDS`$
//! - $`LR`$: クエリ $`[L, R)`$
//!
//! ## verify
//! [ABC174F](https://atcoder.jp/contests/abc174/submissions/31141642)

use crate::{
    data_structure::binary_indexed_tree::BinaryIndexedTree,
    prelude::{binary_operation::addition::Addition, *},
};

const KINDS: usize = 500_000;

pub fn solve(n: usize, q: usize, a: Vec<usize>, lr: Vec<(usize, usize)>) -> Vec<i64> {
    let mut queries = BinaryHeap::new(); // Reverse((r, l, i))
    (0..q).for_each(|i| queries.push(Reverse((lr[i].1, lr[i].0, i))));
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
            ans[query.2] = bit.fold(query.1..query.0);
        }
    }
    ans
}

#[test]
fn test() {
    assert_eq!(
        vec![2, 3, 1],
        solve(4, 3, vec![1, 2, 1, 3], vec![(0, 3), (1, 4), (2, 3)])
    );

    assert_eq!(
        vec![1, 2, 2, 1, 2, 2, 6, 3, 3, 3],
        solve(
            10,
            10,
            vec![2, 5, 6, 5, 2, 1, 7, 9, 7, 2],
            vec![
                (4, 5),
                (1, 4),
                (5, 7),
                (1, 2),
                (6, 8),
                (6, 9),
                (0, 8),
                (5, 9),
                (7, 10),
                (5, 8)
            ]
        )
    )
}
