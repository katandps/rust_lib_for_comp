//! # 数列のXor基底
use crate::prelude::*;

macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}

#[snippet(name = "xor_basis", doc_hidden)]
pub fn xor_basis(src: &[usize]) -> Vec<usize> {
    let mut base = Vec::new();
    for mut v in src.iter().copied() {
        for &b in &base {
            chmin!(v, v ^ b);
        }
        if v > 0 {
            base.push(v);
        }
    }
    base
}
