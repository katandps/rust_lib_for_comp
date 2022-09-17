//! # 平方根
//! (二乗して$x$以下になる最大の値, 二乗して$x$以上になる最小の値)
use crate::prelude::*;

#[snippet(name = "sqrt", doc_hidden)]
pub fn sqrt(a: i64) -> (i64, i64) {
    let x = (a as f64).sqrt() as i64;
    match a.cmp(&(x * x)) {
        Ordering::Greater => (x, x + 1),
        Ordering::Less => (x - 1, x),
        Ordering::Equal => (x, x),
    }
}

#[test]
fn test() {
    for i in 1..30000000 {
        let (l, u) = sqrt(i);
        assert!(l * l <= i);
        assert!((l + 1) * (l + 1) > i);
        assert!(u * u >= i);
        assert!((u - 1) * (u - 1) < i);
    }
}
