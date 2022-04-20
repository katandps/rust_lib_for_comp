//! # Static Range Frequency (整数の出現回数)
//!
//! ## 入力
//! ```math
//! \begin{array}{}
//! N & Q \\
//! a_0 & a_1 & a_2 & \cdots & a_{n-1} \\
//! l_0 & r_0 & x_0 \\
//! l_1 & r_1 & x_1 \\
//! \vdots \\
//! l_{q-1} & r_{q-1} & x_{q-1} \\
//! \end{array}
//! ```
//!

use crate::algo::slice_bounds::SliceBounds;

#[allow(clippy::many_single_char_names)]
pub fn solve(_n: usize, q: usize, a: Vec<usize>, lrx: Vec<(usize, usize, usize)>) -> Vec<usize> {
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
        .collect()
}

#[test]
fn test() {
    assert_eq!(
        vec![2, 0, 1],
        solve(
            5,
            3,
            vec![3, 7, 1, 2, 1],
            vec![(1, 5, 1), (3, 3, 0), (0, 4, 3)]
        )
    );
}
