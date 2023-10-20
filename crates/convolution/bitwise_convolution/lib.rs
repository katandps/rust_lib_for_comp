//! # 高速アダマール変換 (Fast Walsh Hadamard Transform)

use mod_int::ModInt;
use prelude::*;

macro_rules! max {($a:expr $(,)*) => {{$a}};($a:expr, $b:expr $(,)*) => {{if $a > $b {$a} else {$b}}};($a:expr, $($rest:expr),+ $(,)*) => {{let b = max!($($rest),+);if $a > b {$a} else {b}}};}

pub mod and_convolution;
pub mod or_convolution;
pub mod xor_convolution;

#[snippet(name = "bitwise-convolution", doc_hidden)]
pub trait ConvolutionType {
    fn fwht<const M: u32>(poly: &mut [ModInt<M>], rev: bool);
}

#[snippet(name = "bitwise-convolution", doc_hidden)]
pub fn convolution<T: ConvolutionType, const M: u32>(
    mut a: Vec<ModInt<M>>,
    mut b: Vec<ModInt<M>>,
) -> Vec<ModInt<M>> {
    let n = max!(a.len(), b.len()).next_power_of_two();
    a.resize(n, ModInt::zero());
    b.resize(n, ModInt::zero());
    T::fwht(&mut a, false);
    T::fwht(&mut b, false);
    for i in 0..n {
        a[i] *= b[i];
    }
    T::fwht(&mut a, true);
    a.resize(n, ModInt::zero());
    a
}
