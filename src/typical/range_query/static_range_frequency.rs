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

use crate::{algo::slice_bounds::SliceBounds, prelude::*};

pub fn solve<R: BufRead, W: Write, F: FnMut() -> R>(mut reader: Reader<F>, mut writer: Writer<W>) {
    let (n, q) = reader.v2::<usize, usize>();
    let a = reader.vec::<usize>(n);
    let mut a = a
        .into_iter()
        .enumerate()
        .map(|(i, ai)| (ai, i))
        .collect::<Vec<_>>();
    a.sort_unstable(); // 値が小さいほうから並び、同じ値ならindexが小さいほうが左になるようなソート
    for _ in 0..q {
        let (l, r, x) = reader.v3::<usize, usize, usize>();
        writer.ln(a.lower_bound(&(x, r)) - a.lower_bound(&(x, l)));
    }
}

#[test]
fn test() {
    let mut buf = Vec::new();
    solve(
        Reader::new(|| &b"5 3\n3 7 1 2 1\n1 5 1\n3 3 0\n0 4 3\n"[..]),
        Writer::new(&mut buf),
    );
    assert_eq!(Ok("2\n0\n1\n"), std::str::from_utf8(&buf));
}
