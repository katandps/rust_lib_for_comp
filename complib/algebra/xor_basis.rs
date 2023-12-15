//! # 数列のXor基底

pub(crate) use crate::min_max_macro::{chmin, min};

#[codesnip::entry("xor-basis", include("chmin"))]
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
